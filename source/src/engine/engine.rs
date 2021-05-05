/*
    Most basic implementation of matching engine.
*/

use std::collections::LinkedList;
use std::vec::Vec;
use crate::types::types::{Price, Order, OrderId, Execution};

struct OrderIn {
    order: Order,
    id: OrderId,
}

pub struct Engine {
    bids: Vec<OrderIn>,
    asks: Vec<OrderIn>,
    id: OrderId,
    execution_callback: fn(exec: &Execution),
}

impl Engine {

    // Helpers for cross
    fn hit_ask(bid: Price, ask: Price) -> bool {
        return bid >= ask;
    }

    fn hit_bid(ask: Price, bid: Price) -> bool {
        return ask <= bid;
    }

    // Helpers for queue
    fn priority_ask(ask_new: Price, ask_old: Price) -> bool {
        return ask_new < ask_old;
    }

    fn priority_bid(bid_new: Price, bid_old: Price) -> bool {
        return bid_new > bid_old;
    }

    fn send_execution(order_1: &Order, order_2: &Order, cb: fn(&Execution)) {
        let mut exec = order_1.clone();

        // Call callback now
        (cb)(&exec);

        exec.trader = order_2.trader.clone();
        exec.side = !exec.side;

        // Callback for otherside of trade
        (cb)(&exec);
    }

    fn trade(order: &mut Order, matched_order: &mut Order, cb: fn(&Execution)) {
        // Send to execution report now.
        Engine::send_execution(order, matched_order, cb);

        // Completely fill matched
        if order.size >= matched_order.size {
            order.size -= matched_order.size;
            // Removed via retain operation in cross
            matched_order.size = 0;
        }
        // New order completely filled.
        else {
            matched_order.size -= order.size;
            order.size = 0;
        }
    }

    fn cross(&mut self, order: &mut Order) -> bool {
        let isask = order.side;
        let book = if isask { &mut self.bids } else { &mut self.asks };
        let cross_test = if isask { Engine::hit_bid } else { Engine::hit_ask };
        let cb = self.execution_callback;

        for (index, matched_order) in book.iter_mut().enumerate() {
            if !cross_test(order.price, matched_order.order.price) {
                break;
            }

            Engine::trade(order, &mut matched_order.order, cb);
        }

        book.retain(|x| x.order.size > 0);

        order.size == 0
    }

    fn queue(&mut self, order: &mut Order) {
        let isask = order.side;
        let book = if isask { &mut self.asks } else { &mut self.bids };
        let cross_test = if isask { Engine::priority_ask } else { Engine::priority_bid };

        let insertion_index = match book.iter().enumerate().find(|(index, ele)| cross_test(order.price, ele.order.price)).unwrap() {
            (a, b) => a,
            _ => book.len(),
        };
                            
        let new_order = OrderIn { order: order.clone(), id: self.id };
        book.insert(insertion_index, new_order);
    }

    pub fn limit_order(&mut self, order: &mut Order) -> OrderId {
        // Cross off as many shares as possible.
        if !self.cross(order) {
            // Queue order if all shares not crossed off.
            self.queue(order);
        }
        let return_id = self.id;
        self.id += 1;
        return_id
    }

    pub fn cancel(&mut self, order_in: OrderIn) {
        self.asks.retain(|x| x.id != order_in.id);
        self.bids.retain(|x| x.id != order_in.id);
    }

}