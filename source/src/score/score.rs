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
            let mut clone = flow[idx].clone();
            engine.limit_order(&mut clone);
        }
    }
}

pub fn playback(flow: &mut Vec<Order>) {
    let msg_batch_size = 10;

    let mut engine = Engine::new();
    
    let mut batch = msg_batch_size;
    while batch < flow.len() {
        feed(batch - msg_batch_size, batch, &mut engine, flow);

        batch += msg_batch_size;
    }
}