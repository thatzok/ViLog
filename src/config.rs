use rumqttc::v5::MqttOptions;
use serde::Deserialize;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub mqtt: Option<MqttConfig>,
    pub topics: Option<TopicsConfig>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct TopicsConfig {
    pub error: Option<String>,
    pub warning: Option<String>,
    pub service: Option<String>,
    pub info: Option<String>,
    pub status: Option<String>,
    pub command_topic: Option<String>,
    pub command_payload: Option<String>,
    pub command_interval_secs: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct TopicsResolved {
    pub error: String,
    pub warning: String,
    pub service: String,
    pub info: String,
    pub status: String,
    pub command_topic: String,
    pub command_payload: String,
    pub command_interval_secs: u64,
}

pub fn read_app_config() -> Option<AppConfig> {
    use std::env;

    // standard-path: ./vilog.toml; override with ENV `VILOG_CONFIG`
    let path = env::var("VILOG_CONFIG").unwrap_or_else(|_| "vilog.toml".to_string());

    let Ok(raw) = fs::read_to_string(&path) else {
        log::warn!("Keine config file found at  '{}'; using defaults.", path);
        return None;
    };

    match toml::from_str::<AppConfig>(&raw) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            log::warn!("error parsing '{}': {} — using defaults.", path, e);
            None
        }
    }
}

pub fn create_mqtt_options(cfg: Option<&MqttConfig>) -> MqttOptions {
    let default_client_id = "vilogger".to_string();
    let default_host = "127.0.0.1".to_string();
    let default_port: u16 = 1883;
    let default_username = "vilogger".to_string();
    let default_password = "".to_string();
    let default_keep_alive_secs: u64 = 30;

    let client_id = cfg
        .and_then(|c| c.client_id.clone())
        .unwrap_or(default_client_id);
    let host = cfg.and_then(|c| c.host.clone()).unwrap_or(default_host);
    let port = cfg.and_then(|c| c.port).unwrap_or(default_port);

    let mut mqtt_options = MqttOptions::new(client_id, host, port);

    let username = cfg
        .and_then(|c| c.username.clone())
        .unwrap_or(default_username);
    let password = cfg
        .and_then(|c| c.password.clone())
        .unwrap_or(default_password);
    mqtt_options.set_credentials(username, password);

    let keep_alive_secs = cfg
        .and_then(|c| c.keep_alive_secs)
        .unwrap_or(default_keep_alive_secs);
    mqtt_options.set_keep_alive(Duration::from_secs(keep_alive_secs));

    mqtt_options
}

pub fn resolve_topics(cfg: Option<&TopicsConfig>) -> TopicsResolved {
    let defaults = TopicsResolved {
        error: "open3e/680_266_ErrorDtcHistory".to_string(),
        warning: "open3e/680_264_WarningDtcHistory".to_string(),
        service: "open3e/680_262_ServiceDtcHistory".to_string(),
        info: "open3e/680_260_InfoDtcHistory".to_string(),
        status: "open3e/680_258_StatusDtcHistory".to_string(),
        command_topic: "open3e/cmnd".to_string(),
        command_payload: "{\"mode\": \"read-json\", \"data\":[258,260,262,264,266]}".to_string(),
        command_interval_secs: 60,
    };

    TopicsResolved {
        error: cfg.and_then(|c| c.error.clone()).unwrap_or(defaults.error),
        warning: cfg
            .and_then(|c| c.warning.clone())
            .unwrap_or(defaults.warning),
        service: cfg
            .and_then(|c| c.service.clone())
            .unwrap_or(defaults.service),
        info: cfg.and_then(|c| c.info.clone()).unwrap_or(defaults.info),
        status: cfg
            .and_then(|c| c.status.clone())
            .unwrap_or(defaults.status),
        command_topic: cfg
            .and_then(|c| c.command_topic.clone())
            .unwrap_or(defaults.command_topic),
        command_payload: cfg
            .and_then(|c| c.command_payload.clone())
            .unwrap_or(defaults.command_payload),
        command_interval_secs: cfg
            .and_then(|c| c.command_interval_secs)
            .unwrap_or(defaults.command_interval_secs),
    }
}
