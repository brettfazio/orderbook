use csv;

use crate::types::{Order};

pub fn get_raw_feed() -> Vec<Order> {
    let mut flow = Vec::new();

    let path = "./data/score_feed.csv";

    let mut reader = match csv::Reader::from_path(path) {
        Err(_) => return vec![],
        Ok(f) => f,
    };
    
    for result in reader.deserialize() {
        let record: Order = match result {
            Err(_) => continue,
            Ok(f) => f,
        };

        flow.push(record);
    }

    flow
}