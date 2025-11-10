#[path = "../src/dtc.rs"]
mod dtc;

use std::fs;

#[test]
fn read_258_1_json_into_response_dtc() {
    let json_path = "tests/testdata/258_1.json";
    let data = fs::read_to_string(json_path).expect("failed to read test JSON file");

    let parsed: dtc::ResponseDtc =
        serde_json::from_str(&data).expect("failed to deserialize JSON into ResponseDtc");

    assert_eq!(parsed.count, 10, "Count should be 10");
    assert_eq!(
        parsed.list_entries.len(),
        10,
        "ListEntries length should be 10"
    );

    let first = &parsed.list_entries[0];
    assert_eq!(first.state.id, 118);
    assert_eq!(first.state.text, "FourThreeWayValveInternalBufferPosition");
    assert_eq!(first.unknown, 1);
    assert_eq!(first.date_time.date_time, "2025-11-10 10:12:29");
    assert_eq!(first.date_time.timestamp, 1762765949000);
}

#[test]
fn read_266_1_json_into_response_dtc() {
    let json_path = "tests/testdata/266_1.json";
    let data = std::fs::read_to_string(json_path).expect("failed to read test JSON file");

    let parsed: dtc::ResponseDtc =
        serde_json::from_str(&data).expect("failed to deserialize JSON into ResponseDtc");

    assert_eq!(parsed.count, 0, "Count should be 0");
    assert_eq!(parsed.grand_total, Some(0), "GrandTotal should be Some(0)");
    assert!(
        parsed.list_entries.is_empty(),
        "ListEntries should be empty"
    );
}
