use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseDtc {
    pub count: u32,
    pub grand_total: Option<u32>,
    #[serde(default)]
    pub list_entries: Vec<ListEntryDtc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ListEntryDtc {
    #[serde(alias = "Info", alias = "Service", alias = "Warning", alias = "Error")]
    pub state: State,
    pub date_time: EntryDateTime,
    pub unknown: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    #[serde(rename = "ID")]
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct EntryDateTime {
    pub date_time: String, // z. B. "2025-11-09 18:50:06"
    pub timestamp: i64,    // z. B. 1762710606000 unix timestamp in milliseconds)
}

/// Returns all `list_entries` that are contained in `new_message` but not in `old_message`.
pub fn list_entries_new_not_in_old(
    old_message: &ResponseDtc,
    new_message: &ResponseDtc,
) -> Vec<ListEntryDtc> {
    new_message
        .list_entries
        .iter()
        .cloned()
        .filter(|entry| !old_message.list_entries.contains(entry))
        .collect()
}

/// Sorts the given vector of `ListEntryDtc` by their `timestamp` (ascending).
/// This function sorts the vector in-place.
pub fn sort_entries_by_timestamp(entries: &mut Vec<ListEntryDtc>) {
    entries.sort_by_key(|e| e.date_time.timestamp);
}
