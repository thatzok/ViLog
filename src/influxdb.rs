use crate::config::InfluxResolved;
use std::sync::Arc;
use chrono::prelude::{DateTime, Utc};

pub async fn send_to_influx(
    client: reqwest::Client,
    influx: Arc<InfluxResolved>,
    body: String,
) -> Result<(), reqwest::Error> {
    let mut url = influx.url.clone();
    if url.ends_with('/') {
        url.pop();
        while url.ends_with('/') {
            url.pop();
        }
    }
    let full_url = format!("{}/api/v2/write", url);

    let mut req = client
        .post(full_url)
        .query(&[
            ("org", influx.org.as_str()),
            ("bucket", influx.bucket.as_str()),
            ("precision", "ms"),
        ])
        .header("Content-Type", "text/plain; charset=utf-8");
    if !influx.token.is_empty() {
        req = req.header("Authorization", format!("Token {}", influx.token));
    }
    let resp = req.body(body).send().await?;
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    #[cfg(debug_assertions)]
    {
        println!("InfluxDB write returned {}: {}", status, text);
    }

    if !status.is_success() {
        log::warn!("InfluxDB write returned {}: {}", status, text);
    }
    Ok(())
}

pub fn escape_measurement(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            ',' | ' ' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

pub fn escape_tag(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            ',' | ' ' | '=' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

pub fn escape_field_string(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 2);
    out.push('"');
    for ch in input.chars() {
        match ch {
            '"' => {
                out.push('\\');
                out.push('"');
            }
            '\\' => {
                out.push('\\');
                out.push('\\');
            }
            '\n' | '\r' => {
                out.push(' ');
            }
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

pub fn timestamp_to_datetime_string(timestamp: i64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp_millis(timestamp)
        .expect("Ungültiger Timestamp");

    format!("{}", datetime.format("%+"))
}
