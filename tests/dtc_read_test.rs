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
    let data = fs::read_to_string(json_path).expect("failed to read test JSON file");

    let parsed: dtc::ResponseDtc =
        serde_json::from_str(&data).expect("failed to deserialize JSON into ResponseDtc");

    assert_eq!(parsed.count, 0, "Count should be 0");
    assert_eq!(parsed.grand_total, Some(0), "GrandTotal should be Some(0)");
    assert!(
        parsed.list_entries.is_empty(),
        "ListEntries should be empty"
    );
}

#[test]
fn diff_new_entries_between_258_1_and_258_2() {
    let old_data =
        fs::read_to_string("tests/testdata/258_1.json").expect("failed to read 258_1.json");
    let old_message: dtc::ResponseDtc =
        serde_json::from_str(&old_data).expect("failed to parse 258_1.json");

    let new_data =
        fs::read_to_string("tests/testdata/258_2.json").expect("failed to read 258_2.json");
    let new_message: dtc::ResponseDtc =
        serde_json::from_str(&new_data).expect("failed to parse 258_2.json");

    let diff = dtc::list_entries_new_not_in_old(&old_message, &new_message);

    assert_eq!(diff.len(), 1, "Expected exactly one new entry");

    let entry = &diff[0];
    assert_eq!(entry.state.id, 129);
    assert_eq!(entry.state.text, "HeatPumpPostRun");
    assert_eq!(entry.unknown, 1);
    assert_eq!(entry.date_time.date_time, "2025-11-10 11:17:24");
    assert_eq!(entry.date_time.timestamp, 1762769844000);
}

#[test]
fn diff_new_entries_between_258_1_and_258_3() {
    let old_data =
        fs::read_to_string("tests/testdata/258_1.json").expect("failed to read 258_1.json");
    let old_message: dtc::ResponseDtc =
        serde_json::from_str(&old_data).expect("failed to parse 258_1.json");

    let new_data =
        fs::read_to_string("tests/testdata/258_3.json").expect("failed to read 258_3.json");
    let new_message: dtc::ResponseDtc =
        serde_json::from_str(&new_data).expect("failed to parse 258_3.json");

    let diff = dtc::list_entries_new_not_in_old(&old_message, &new_message);

    assert_eq!(diff.len(), 3, "Expected exactly three new entries");

    let e0 = &diff[0];
    assert_eq!(e0.state.id, 134);
    assert_eq!(e0.state.text, "FourThreeWayValveIdlePosition");
    assert_eq!(e0.unknown, 1);
    assert_eq!(e0.date_time.date_time, "2025-11-10 11:19:28");
    assert_eq!(e0.date_time.timestamp, 1762769968000);

    let e1 = &diff[1];
    assert_eq!(e1.state.id, 123);
    assert_eq!(e1.state.text, "HeatPumpOff");
    assert_eq!(e1.unknown, 1);
    assert_eq!(e1.date_time.date_time, "2025-11-10 11:19:25");
    assert_eq!(e1.date_time.timestamp, 1762769965000);

    let e2 = &diff[2];
    assert_eq!(e2.state.id, 129);
    assert_eq!(e2.state.text, "HeatPumpPostRun");
    assert_eq!(e2.unknown, 1);
    assert_eq!(e2.date_time.date_time, "2025-11-10 11:17:24");
    assert_eq!(e2.date_time.timestamp, 1762769844000);
}
