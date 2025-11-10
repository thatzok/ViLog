use std::fs;

mod dtc;

fn main() {
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
        println!("{},{},{},{}", e.date_time.timestamp, e.date_time.date_time, e.state.id, e.state.text);
    }
}

