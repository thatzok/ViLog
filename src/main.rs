use std::fs;

use std::error::Error;

use rumqttc::v5::mqttbytes::QoS;
use tokio::{task, time};

use rumqttc::v5::AsyncClient;

use std::time::Duration;

mod config;
mod dtc;
use crate::config::create_mqtt_options;

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

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
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
        .subscribe("open3e/680_258_StatusDtcHistory", QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();

    // Alle 30 Sekunden eine Nachricht veröffentlichen
    let mut ticker = time::interval(Duration::from_secs(30));
    loop {
        ticker.tick().await;
        match client
            .publish("open3e/cmnd", QoS::ExactlyOnce, false, "{\"mode\": \"read-json\", \"data\":[258]}")
            .await
        {
            Ok(_) => log::info!("Published keep-alive message to 'open3e/cmnd'"),
            Err(e) => log::warn!("Publish failed: {e}"),
        }
    }
}
