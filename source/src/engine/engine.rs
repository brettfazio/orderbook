/*
    Most basic implementation of matching engine.
*/

use std::collections::LinkedList;
use std::vec::Vec;
use crate::types::types::{Price, Order};

pub struct Engine {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub id: u32,
}

impl Engine {

    // Helpers
    fn hit_ask(bid: Price, ask: Price) -> bool {
        return bid >= ask;
    }

    fn hit_bid(ask: Price, bid: Price) -> bool {
        return ask <= bid;
    }

    fn trade(order: &mut Order, matched_order: &mut Order) {
        // Send to execution report now.

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

        for (index, matched_order) in book.iter_mut().enumerate() {
            if !cross_test(order.price, matched_order.price) {
                break;
            }

            Engine::trade(order, matched_order);
        }

        book.retain(|x| x.size > 0);

        order.size == 0
    }

    fn queue(&self, order: &mut Order) -> bool {

        false
    }

    pub fn limit_order(&mut self, order: &mut Order) {
        // Cross off as many shares as possible.
        if !self.cross(order) {
            // Queue order if all shares not crossed off.
            self.queue(order);
        }
    }

}