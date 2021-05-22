/*
    Reimplementation of the winning QuantCup 1 implementation
    https://gist.github.com/druska/d6ce3f2bac74db08ee9007cdf98106ef
*/

use std::vec::Vec;
use std::collections::VecDeque;
use core::cmp::min;
use crate::types::{Order, Price, OrderId, Execution, is_ask};

pub struct OrderIn {
    order: Order,
    id: OrderId,
}

struct PricePoint {
    items: VecDeque<OrderId>
}

pub struct Engine {
    ask_min: Price,
    bid_max: Price,
    book_entries: Vec<OrderIn>,
    price_points: Vec<PricePoint>,
    id: OrderId,
    pub execution_log: Vec<Execution>,
    should_log: bool
}

impl Engine {

    fn _new(debug: bool) -> Engine {
        let max_orders = 1010000;

        let mut pps: Vec<PricePoint> = Vec::with_capacity((Price::max_value() as usize) + 1);

        let mut idx = 0;
        while idx < (Price::max_value() as usize) + 1 {
            pps.push(PricePoint{ items: VecDeque::new() });
            idx += 1;
        }

        Engine {
            ask_min: 0,
            bid_max: 0,
            book_entries: Vec::with_capacity(max_orders as usize),
            price_points: pps,
            id: 1,
            execution_log: Vec::new(),
            should_log: debug
        }
    }

    pub fn new() -> Engine {
        Engine::_new(false)
    }

    pub fn new_debug() -> Engine {
        Engine::_new(true)
    }
    
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

    // Original implementation used an undefined header function in the engine.h 
    // to be implemented should you want the backlog of orders to confirm the engine is valid.
    // The original implementation only implements this function when the engine is being tested - not scored.
    // Thus when the engine is being tested should_log is true, and when it is being scored should_log is false.
    fn send_execution(order_1: &Order, order_2: &Order, log: &mut Vec<Execution>) {
        let mut exec = order_1.clone();
        exec.size = min(order_1.size, order_2.size);

        // Call callback now
        log.push(exec.clone());

        exec.trader = order_2.trader.clone();
        exec.side = exec.side ^ 1;

        // Callback for otherside of trade
        log.push(exec);
    }

    fn trade(order: &mut Order, matched_order: &mut Order, log: &mut Vec<Execution>, should_log: bool) {
        if should_log {
            // Send to execution report now.
            Engine::send_execution(&order.clone(), &matched_order.clone(), log);
        }
        

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
        true
    }

    fn queue(&mut self, order: Order) {
        // Add to price point.
        self.price_points[order.price as usize].items.push_back(self.id);
        // Add to book entries.
        self.book_entries[self.id as usize] = OrderIn { order: order, id: self.id };
    }

    pub fn limit_order(&mut self, mut order: Order) -> OrderId {
        // Cross off as many shares as possible.
        if is_ask(order.side) {
            if order.price >= self.ask_min {
                let pp_entry = &mut self.price_points[self.ask_min as usize];

                loop {
                    let mut entries = &mut pp_entry.items;

                    // Go over entries
                    for item_id in entries.iter_mut() {
                        let mut item = &mut self.book_entries[*item_id as usize].order;
                        if item.size < order.size {
                            Engine::trade(&mut order, item, &mut self.execution_log, self.should_log);

                            if order.size == 0 {
                                break;
                            }
                        }
                    }
                    // Remove
                    loop {
                        match entries.front() {
                            Some(x) => {
                                if self.book_entries[*x as usize].order.size == 0 {
                                    entries.pop_front();
                                }else {
                                    break;
                                }
                            }
                            None => break
                        }
                    }
                    
                    if order.size == 0 {
                        let return_id = self.id;
                        self.id += 1;
                        return return_id;
                    }

                    // All orders at the current price point.
                    self.ask_min += 1;
                    if order.price < self.ask_min {
                        break;
                    }
                }

                // Adjust potential max
                if self.bid_max < order.price {
                    self.bid_max = order.price;
                }
                // Queue order
                self.queue(order);
                // Return new order number
                let return_id = self.id;
                self.id += 1;
                return return_id;
            }
        }
        else {

        }

        5
    }

    pub fn cancel(&mut self, id: OrderId) {
        self.
    }

}