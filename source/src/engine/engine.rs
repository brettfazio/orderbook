/*
    Most basic implementation of matching engine.
*/

use std::collections::LinkedList;
use crate::types::types::{Price, Order};

pub struct Engine {
    pub bids: LinkedList<Order>,
    pub asks: LinkedList<Order>,
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

    fn trade(&mut self, order: &mut Order, matched_order: &mut Order, book: &LinkedList<Order>) {

    }

    fn cross(&mut self, order: &mut Order) -> bool {
        let isask = order.side;
        let book = if isask { &mut self.bids } else { &mut self.asks };
        let cross_test = if isask { Engine::hit_bid } else { Engine::hit_ask };

        let mut book_iter = book.iter_mut();
        let mut next = &mut book_iter.next();
        while !next.is_none() && cross_test(order.price, next.as_ref().unwrap().price) {
            self.trade(order, next.unwrap(), book);

            // Executed whole trade.
            if order.size == 0 {
                return true;
            }

            next = &mut book_iter.next();
        }

        false
    }

    fn queue(&self, order: &mut Order) -> bool {

        false
    }

    pub fn limit_order(&self, order: &mut Order) {
        // Cross off as many shares as possible.
        if !self.cross(order) {
            // Queue order if all shares not crossed off.
            self.queue(order);
        }
    }

}