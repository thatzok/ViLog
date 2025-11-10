use serde::Deserialize;
use std::fs;
use std::time::Duration;

use rumqttc::v5::MqttOptions;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub mqtt: Option<MqttConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub client_id: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub keep_alive_secs: Option<u64>,
}

pub fn load_mqtt_config() -> Option<MqttConfig> {
    use std::env;

    // Standard-Pfad: ./vilog.toml; override über ENV `VILOG_CONFIG`
    let path = env::var("VILOG_CONFIG").unwrap_or_else(|_| "vilog.toml".to_string());

    let Ok(raw) = fs::read_to_string(&path) else {
        log::warn!("Keine Konfigurationsdatei gefunden unter '{}'; verwende Defaults.", path);
        return None;
    };

    match toml::from_str::<AppConfig>(&raw) {
        Ok(cfg) => cfg.mqtt,
        Err(e) => {
            log::warn!("Fehler beim Parsen von '{}': {} — verwende Defaults.", path, e);
            None
        }
    }
}

pub fn create_mqtt_options() -> MqttOptions {

    let default_client_id = "vilogger".to_string();
    let default_host = "192.168.112.12".to_string();
    let default_port: u16 = 1883;
    let default_username = "vilogger".to_string();
    let default_password = "".to_string();
    let default_keep_alive_secs: u64 = 30;

    let cfg = load_mqtt_config();

    let client_id = cfg
        .as_ref()
        .and_then(|c| c.client_id.clone())
        .unwrap_or(default_client_id);
    let host = cfg
        .as_ref()
        .and_then(|c| c.host.clone())
        .unwrap_or(default_host);
    let port = cfg
        .as_ref()
        .and_then(|c| c.port)
        .unwrap_or(default_port);

    let mut mqtt_options = MqttOptions::new(client_id, host, port);

    let username = cfg
        .as_ref()
        .and_then(|c| c.username.clone())
        .unwrap_or(default_username);
    let password = cfg
        .as_ref()
        .and_then(|c| c.password.clone())
        .unwrap_or(default_password);
    mqtt_options.set_credentials(username, password);


    let keep_alive_secs = cfg
        .as_ref()
        .and_then(|c| c.keep_alive_secs)
        .unwrap_or(default_keep_alive_secs);
    mqtt_options.set_keep_alive(Duration::from_secs(keep_alive_secs));

    //        .mqtt_version(rumqttc::MqttVersion::V5);

    mqtt_options
}
