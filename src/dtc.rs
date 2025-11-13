use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseDtc {
    pub count: u32,
    pub grand_total: Option<u32>,
    #[serde(default)]
    pub list_entries: Vec<ListEntryDtc>,
}
impl ResponseDtc {
    pub fn new_empty() -> Self {
        Self {
            count: 0,
            grand_total: None,
            list_entries: Vec::new(),
        }
    }
}

/*
    I Info      Informationen           info
    P Service   Wartungsmeldungen       notice
    S State     Statusmeldungen         debug
    A Warning   Warnungsmeldungen       warning
    F Error     Störungsmeldungen       err
*/

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListEntryDtc {
    #[serde(skip_serializing)]
    /// Holds the original field name used in the source (one of: "Info", "State", "Service", "Warning", "Error").
    pub state_type: String,
    pub state: State,
    pub date_time: EntryDateTime,
    pub unknown: i64,
}

// Custom deserialization to capture which alias field name (Info/State/Service/Warning/Error)
// was actually present in the payload
impl<'de> serde::Deserialize<'de> for ListEntryDtc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Helper {
            #[serde(rename = "State")]
            state: Option<State>,
            #[serde(rename = "Info")]
            info: Option<State>,
            #[serde(rename = "Service")]
            service: Option<State>,
            #[serde(rename = "Warning")]
            warning: Option<State>,
            #[serde(rename = "Error")]
            error: Option<State>,
            date_time: EntryDateTime,
            unknown: i64,
        }

        let h = Helper::deserialize(deserializer)?;

        // Determine which one is set
        let mut which = None::<(&str, State)>;
        if let Some(s) = h.info.clone() {
            which = Some(("Info", s));
        }
        if let Some(s) = h.service.clone() {
            if which.is_some() {
                return Err(serde::de::Error::custom(
                    "More then one of (Info/Service/Warning/Error/State) found",
                ));
            }
            which = Some(("Service", s));
        }
        if let Some(s) = h.warning.clone() {
            if which.is_some() {
                return Err(serde::de::Error::custom(
                    "More then one of (Info/Service/Warning/Error/State) found",
                ));
            }
            which = Some(("Warning", s));
        }
        if let Some(s) = h.error.clone() {
            if which.is_some() {
                return Err(serde::de::Error::custom(
                    "More then one of (Info/Service/Warning/Error/State) found",
                ));
            }
            which = Some(("Error", s));
        }
        if let Some(s) = h.state.clone() {
            if which.is_some() {
                return Err(serde::de::Error::custom(
                    "More then one of (Info/Service/Warning/Error/State) found",
                ));
            }
            which = Some(("State", s));
        }

        let (state_type, state) = match which {
            Some((name, s)) => (name.to_string(), s),
            None => {
                return Err(serde::de::Error::custom(
                    "No one of (Info/Service/Warning/Error/State) found",
                ));
            }
        };

        Ok(ListEntryDtc {
            state_type,
            state,
            date_time: h.date_time,
            unknown: h.unknown,
        })
    }
}

impl PartialEq for ListEntryDtc {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.date_time == other.date_time
            && self.unknown == other.unknown
    }
}
impl Eq for ListEntryDtc {}

impl ListEntryDtc {
    pub fn get_iso8601_from_timestamp(&self) -> String {
        let datetime = DateTime::<Utc>::from_timestamp_millis(self.date_time.timestamp)
            .expect("Invalid timestamp");
        format!("{}", datetime.format("%+"))
    }

    pub fn get_severity(&self) -> String {
        match self.state_type.as_str() {
            "Info" => "info".to_string(),
            "Service" => "notice".to_string(),
            "State" => "debug".to_string(),
            "Warning" => "warning".to_string(),
            "Error" => "err".to_string(),
            _ => "info".to_string(),
        }
    }

    pub fn get_msg_code_letter(&self) -> String {
        match self.state_type.as_str() {
            "Info" => 'I'.to_string(),
            "Service" => 'P'.to_string(),
            "State" => 'S'.to_string(),
            "Warning" => 'A'.to_string(),
            "Error" => 'F'.to_string(),
            _ => 'I'.to_string(),
        }
    }

    pub fn get_msg_code(&self) -> String {
        format!(
            "{}.{}",
            self.get_msg_code_letter(),
            self.state.id.to_string()
        )
    }
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
