use std::error::Error;
use std::time::Duration;

use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::AsyncClient;
use rumqttc::v5::{Event, Incoming};
use tokio::{task, time};

mod config;
mod dtc;
use crate::config::{create_mqtt_options, read_app_config, TopicsResolved};
use crate::dtc::{ListEntryDtc, ResponseDtc};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let app_cfg = read_app_config();

    let mqtt_cfg_opt = app_cfg.as_ref().and_then(|c| c.mqtt.as_ref());
    let mqttoptions = create_mqtt_options(mqtt_cfg_opt);

    let topics_cfg_opt = app_cfg.as_ref().and_then(|c| c.topics.as_ref());
    let topics = config::resolve_topics(topics_cfg_opt);

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

                if topic_str == topics.error.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_error_message, &new_message);
                    old_error_message = new_message;
                } else if topic_str == topics.warning.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_warning_message, &new_message);
                    old_warning_message = new_message;
                } else if topic_str == topics.service.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_service_message, &new_message);
                    old_service_message = new_message;
                } else if topic_str == topics.info.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_info_message, &new_message);
                    old_info_message = new_message;
                } else if topic_str == topics.status.as_str() {
                    diff = dtc::list_entries_new_not_in_old(&old_status_message, &new_message);
                    old_status_message = new_message;
                }

                dtc::sort_entries_by_timestamp(&mut diff);

                // println!("timestamp,date_time,id,text");
                for e in diff {
                    println!(
                        "{},{},{},{},{}",
                        topic_str,
                        e.date_time.timestamp,
                        e.date_time.date_time,
                        e.state.id,
                        e.state.text
                    );
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
