use std::time::{Instant, Duration};

use crate::feed::feed::{get_raw_feed};
use crate::engine::engine::Engine;
use crate::types::Order;


fn feed(begin: usize, end: usize, engine: &mut Engine, flow: &mut Vec<Order>) {
    for idx in begin..end {
        // Raw simulated order and cancel data feed
        // orders with price = 0 correspond to 
        // cancels with orderid=size
        if flow[idx].price == 0 {
            engine.cancel(flow[idx].size);
        }else {
            engine.limit_order(&mut flow[idx]);
        }
    }
}

pub fn get_score() -> u32 {
    let replays = 200;
    let msg_batch_size = 10;

    let mut flow = get_raw_feed();

    for replay in 0..replays {
        let mut engine = Engine::new();

        let mut batch = 0;
        while batch < msg_batch_size {

            let begin = Instant::now();

            feed(batch - msg_batch_size, batch, &mut engine, &mut flow);

            let elapsed = begin.elapsed();

            batch += msg_batch_size;
        }

    }

    0
}