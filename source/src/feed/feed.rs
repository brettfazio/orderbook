use csv;

use crate::types::{Order};

fn get_raw_feed() -> Vec<Order> {
    /* Raw simulated order and cancel data feed
    orders with price = 0 correspond to 
    cancels with orderid=size */

    let mut flow = Vec::new();

    let path = "../../data/score_feed.csv";

    let mut reader = match csv::Reader::from_path(path) {
        Err(e) => return vec![],
        Ok(f) => f,
    };
    
    for result in reader.deserialize() {
        let record: Order = match result {
            Err(e) => continue,
            Ok(f) => f,
        };

        flow.push(record);
    }

    flow
}