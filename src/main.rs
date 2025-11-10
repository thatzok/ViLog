use std::fs;

mod dtc;

fn main() {
    println!("Hello, world!");

    let old_data =
        fs::read_to_string("tests/testdata/258_1.json").expect("failed to read 258_1.json");
    let old_message: dtc::ResponseDtc =
        serde_json::from_str(&old_data).expect("failed to parse 258_1.json");

    let new_data =
        fs::read_to_string("tests/testdata/258_3.json").expect("failed to read 258_3.json");
    let new_message: dtc::ResponseDtc =
        serde_json::from_str(&new_data).expect("failed to parse 258_3.json");

    let diff = dtc::list_entries_new_not_in_old(&old_message, &new_message);
    println!("diff: {:?}", diff);
}

