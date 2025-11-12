use std::error::Error;
use std::time::Duration;

use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use rumqttc::v5::{Event, Incoming};
use std::sync::Arc;
use tokio::{task, time};

mod config;
mod dtc;
mod influxdb;

use crate::config::resolve_influx;
use crate::config::{create_mqtt_options, read_app_config, TopicsResolved};
use crate::dtc::{ListEntryDtc, ResponseDtc};
use crate::influxdb::{escape_field_string, escape_measurement, escape_tag, send_to_influx};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    println!("ViLog Version: {}", env!("CARGO_PKG_VERSION"));
    log::info!("ViLog version:  {:?}", env!("CARGO_PKG_VERSION"));

    let app_cfg = read_app_config();

    let mqtt_cfg_opt = app_cfg.as_ref().and_then(|c| c.mqtt.as_ref());
    let mqttoptions = create_mqtt_options(mqtt_cfg_opt);

    let topics_cfg_opt = app_cfg.as_ref().and_then(|c| c.topics.as_ref());
    let topics = config::resolve_topics(topics_cfg_opt);

    // InfluxDB config and HTTP client
    let influx_resolved = resolve_influx(app_cfg.as_ref().and_then(|c| c.influxdb.as_ref()));
    let http_client = if influx_resolved.enabled {
        let timeout = Duration::from_secs(influx_resolved.timeout_secs);
        match reqwest::Client::builder().timeout(timeout).build() {
            Ok(c) => Some(c),
            Err(e) => {
                log::error!("Failed to create HTTP client for InfluxDB: {}", e);
                None
            }
        }
    } else {
        None
    };

    let mut old_info_message = ResponseDtc::new_empty();
    let mut old_status_message = ResponseDtc::new_empty();
    let mut old_service_message = ResponseDtc::new_empty();
    let mut old_warning_message = ResponseDtc::new_empty();
    let mut old_error_message = ResponseDtc::new_empty();

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let topics_for_task = topics.clone();
    task::spawn(async move {
        requests(client, topics_for_task).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let event = eventloop.poll().await;
        match event {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let topic_bytes = p.topic;
                let payload_bytes = p.payload;
                let topic_str = std::str::from_utf8(&topic_bytes).unwrap_or_default();
                let payload_str = std::str::from_utf8(&payload_bytes).unwrap_or_default();

                let new_message: ResponseDtc =
                    serde_json::from_str(payload_str).expect("failed to parse payload");

                let mut diff: Vec<ListEntryDtc> = Vec::new();
                let mut severity = "Debug";

                if topic_str == topics.error.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_error_message, &new_message);
                    old_error_message = new_message;
                    severity = "err";
                } else if topic_str == topics.warning.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_warning_message, &new_message);
                    severity = "warning";
                    old_warning_message = new_message;
                } else if topic_str == topics.service.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_service_message, &new_message);
                    severity = "notice";
                    old_service_message = new_message;
                } else if topic_str == topics.info.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_info_message, &new_message);
                    severity = "info";
                    old_info_message = new_message;
                } else if topic_str == topics.status.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_status_message, &new_message);
                    severity = "debug";
                    old_status_message = new_message;
                }

                dtc::sort_entries_by_timestamp(&mut diff);

                if !diff.is_empty() {
                    if let (Some(client), true) = (&http_client, influx_resolved.enabled) {
                        let mut body = String::with_capacity(diff.len() * 128);
                        for e in &diff {
                            let ts_ms = e.date_time.timestamp; // already in ms 
                            let mut line = String::new();
                            line.push_str(&escape_measurement(&influx_resolved.measurement));
                            // tags
                            line.push(',');
                            line.push_str("systemid=");
                            line.push_str(&escape_tag(&topics.systemid));
                            line.push(',');
                            line.push_str("ecuid=");
                            line.push_str(&escape_tag(&topics.ecuid));
                            line.push(',');
                            line.push_str("severity=");
                            line.push_str(&escape_tag(severity));

                            // fields
                            line.push(' ');
                            line.push_str("textid=");
                            line.push_str(&e.state.id.to_string());
                            line.push('i');
                            line.push(',');
                            line.push_str("text=");
                            line.push_str(&escape_field_string(&e.state.text));
                            // timestamp
                            line.push(' ');
                            line.push_str(&ts_ms.to_string());
                            body.push_str(&line);
                            body.push('\n');
                        }
                        let body_clone = body.clone();
                        let client = client.clone();
                        let influx = Arc::new(influx_resolved.clone());
                        task::spawn(async move {
                            if let Err(err) =
                                send_to_influx(client.clone(), influx.clone(), body_clone).await
                            {
                                log::error!("InfluxDB write failed: {}", err);
                            }
                        });
                    }

                    for e in diff {
                        println!(
                            "{} {} {}[{}]: {} {}",
                            e.date_time.date_time,
                            topics.systemid,
                            topics.ecuid,
                            e.state.id,
                            severity,
                            e.state.text
                        );
                    }
                }
            }
            Ok(other) => {
                // // ignore other events for now
                log::debug!("Event = {:?}", other);
            }
            Err(e) => {
                println!("Error = {e:?}");
                log::error!("Event = {:?}", e);
                return Ok(());
            }
        }
    }
}

async fn requests(client: AsyncClient, topics: TopicsResolved) {
    client
        .subscribe(topics.error.as_str(), QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(topics.warning.as_str(), QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(topics.service.as_str(), QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .subscribe(topics.info.as_str(), QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .subscribe(topics.status.as_str(), QoS::AtMostOnce)
        .await
        .unwrap();

    let mut ticker = time::interval(Duration::from_secs(topics.command_interval_secs));
    loop {
        ticker.tick().await;
        let pub_topic = topics.command_topic.clone();
        let pub_payload = topics.command_payload.clone();
        match client
            .publish(pub_topic.clone(), QoS::ExactlyOnce, false, pub_payload)
            .await
        {
            Ok(_) => log::debug!("Published keep-alive message to '{}'", pub_topic),
            Err(e) => log::error!("Publish failed: {e}"),
        }
    }
}
