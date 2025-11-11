use std::fs;

use std::error::Error;

use rumqttc::v5::mqttbytes::QoS;
use tokio::{task, time};

use rumqttc::v5::AsyncClient;
use rumqttc::v5::{Event, Incoming};

use std::time::Duration;

mod config;
mod dtc;
use crate::config::create_mqtt_options;
use crate::dtc::{ListEntryDtc, ResponseDtc};

fn maintest() {
    let old_data =
        fs::read_to_string("tests/testdata/258_1.json").expect("failed to read 258_1.json");
    let old_message: dtc::ResponseDtc =
        serde_json::from_str(&old_data).expect("failed to parse 258_1.json");

    let new_data =
        fs::read_to_string("tests/testdata/258_3.json").expect("failed to read 258_3.json");
    let new_message: dtc::ResponseDtc =
        serde_json::from_str(&new_data).expect("failed to parse 258_3.json");

    let mut diff = dtc::list_entries_new_not_in_old(&old_message, &new_message);

    dtc::sort_entries_by_timestamp(&mut diff);

    println!("timestamp,date_time,id,text");
    for e in diff {
        println!(
            "{},{},{},{}",
            e.date_time.timestamp, e.date_time.date_time, e.state.id, e.state.text
        );
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut mqttoptions = create_mqtt_options();

    let mut old_info_message = ResponseDtc::new_empty();
    let mut old_status_message = ResponseDtc::new_empty();
    let mut old_service_message = ResponseDtc::new_empty();
    let mut old_warning_message = ResponseDtc::new_empty();
    let mut old_error_message = ResponseDtc::new_empty();

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let event = eventloop.poll().await;
        match event {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                // println!("Pub = {p:?}");
                let topic_bytes = p.topic;
                let payload_bytes = p.payload;

                // println!("topic_bytes: {:?}",topic_bytes);

                let topic_str = std::str::from_utf8(&topic_bytes).unwrap_or_default();
                let payload_str = std::str::from_utf8(&payload_bytes).unwrap_or_default();
                // println!("topic: {}, payload: {}", topic_str, payload_str);
                // println!("topic: {}", topic_str);

                let new_message: dtc::ResponseDtc =
                    serde_json::from_str(payload_str).expect("failed to parse payload");

                let mut diff: Vec<ListEntryDtc> = Vec::new();

                if topic_str == "open3e/680_266_ErrorDtcHistory" {
                    diff = dtc::list_entries_new_not_in_old(&old_error_message, &new_message);
                    old_error_message = new_message;
                } else if topic_str == "open3e/680_264_WarningDtcHistory" {
                    diff = dtc::list_entries_new_not_in_old(&old_warning_message, &new_message);
                    old_warning_message = new_message;
                } else if topic_str == "open3e/680_262_ServiceDtcHistory" {
                    diff = dtc::list_entries_new_not_in_old(&old_service_message, &new_message);
                    old_service_message = new_message;
                } else if topic_str == "open3e/680_260_InfoDtcHistory" {
                    diff = dtc::list_entries_new_not_in_old(&old_info_message, &new_message);
                    old_info_message = new_message;
                } else if topic_str == "open3e/680_258_StatusDtcHistory" {
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
                // Andere Events ignorieren oder ggf. loggen
                log::debug!("Event = {:?}", other);
            }
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    // Abonnements einmalig einrichten

    client
        .subscribe("open3e/680_266_ErrorDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe("open3e/680_264_WarningDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe("open3e/680_262_ServiceDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .subscribe("open3e/680_260_InfoDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .subscribe("open3e/680_258_StatusDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();

    // Alle 30 Sekunden eine Nachricht veröffentlichen
    let mut ticker = time::interval(Duration::from_secs(30));
    let cmd = "{\"mode\": \"read-json\", \"data\":[258,260,262,264,266]}";
    loop {
        ticker.tick().await;
        match client
            .publish("open3e/cmnd", QoS::ExactlyOnce, false, cmd)
            .await
        {
            Ok(_) => log::info!("Published keep-alive message to 'open3e/cmnd'"),
            Err(e) => log::warn!("Publish failed: {e}"),
        }
    }
}
