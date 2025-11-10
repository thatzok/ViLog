use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseDtc {
    pub count: u32,
    pub grand_total: Option<u32>,
    #[serde(default)]
    pub list_entries: Vec<ListEntryDtc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListEntryDtc {
    pub state: State,
    pub date_time: EntryDateTime,
    pub unknown: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    #[serde(rename = "ID")]
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntryDateTime {
    pub date_time: String, // z. B. "2025-11-09 18:50:06"
    pub timestamp: i64,    // z. B. 1762710606000 unix timestamp in milliseconds)
}
