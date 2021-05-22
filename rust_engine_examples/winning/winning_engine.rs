/*
    Reimplementation of the winning QuantCup 1 implementation
    https://gist.github.com/druska/d6ce3f2bac74db08ee9007cdf98106ef
*/

use std::vec::Vec;
use core::cmp::min;
use crate::types::{Order, Price, OrderId, Execution, is_ask};

// Self contained linked list code so the engine code can be dropped right into the orderbook
// codebase.
#[derive(Clone)]
enum Link<T> {
    None,
    Tail { item: T },
    Link { item: T, next: Box<Link<T>> }
}

#[derive(Clone)]
struct Cursor<T> { 
    curr: Link<T>
}

impl<T> Link<T> where T: Copy {
    pub fn new() -> Self {
        Self::None    
    }
    
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Tail { item } => {
              let item = *item;
              self.to_none();
              Some(item)
            },
            Self::Link { item, next } => {
                let mut n = Box::new(Self::None);
                let item = *item;
                std::mem::swap(next, &mut n);
                self.to_next(*n);
                Some(item)
            }
        }
    }
    
    pub fn push(&mut self, x: T) {
        match self {
           Self::None => self.to_tail(x),
           Self::Tail { .. } => self.to_link(x),
           Self::Link { next, .. } => next.push(x)
        };
    }
    
    fn to_none(&mut self) { *self = std::mem::replace(self, Link::None); }
    
    fn to_tail(&mut self, it: T) {
        *self = match self {
            Self::None => Self::Tail { item: it },
            Self::Link { item:_, next:_ } => Self::Tail { item: it },
            _ => panic!("Supplied value was not of correct type or variant.")
        }
    }
    
    fn to_next(&mut self, nxt: Link<T>) {
        *self = nxt;
    }
    
    fn to_link(&mut self, x: T) {
        *self = match self {
            Self::Tail { item } => { 
                Self::Link { item: *item, next: Box::new(Self::Tail { item: x }) }
            },
            _ => { panic!("something went wrong"); }
        };
    }
}

impl<T> IntoIterator for Link<T> where T: Copy {
    type Item = T;
    type IntoIter = Cursor<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        Cursor {
            curr: self
        }
    }
}

impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
        let nxt = match self.curr {
            Link::None => None,
            Link::Tail { item } => {
                self.curr = Link::None;
                Some(item)
            },
            Link::Link { item, ref mut next } => {
                let mut n = Box::new(Link::None);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                Some(item)
            }
        };
        nxt
    }
}

pub struct OrderIn {
    order: Order,
    id: OrderId,
}

struct PricePoint {
    head: Link<Order>,
    tail: Link<Order>,
}

pub struct Engine {
    ask_min: Price,
    bid_max: Price,
    book_entries: Vec<OrderIn>,
    price_points: [PricePoint; Price::max_value()+1],
    id: OrderId,
    pub execution_log: Vec<Execution>,
    should_log: bool
}

impl Engine {

    pub fn new() -> Engine {
        let max_orders = 1010000;

        Engine {
            ask_min: 0,
            bid_max: 0,
            book_entries: Vec::with_capacity(max_orders as usize),
            price_points: [{ head: Link::new(), tail: Link::new() }; Price::max_value()+1],
            id: 1,
            execution_log: Vec::new(),
            should_log: false
        }
    }

    pub fn new_debug() -> Engine {
        Engine {
            ask_min: Price::max_value(),
            bid_max: 1,
            book_entries: Vec::with_capacity(max_orders as usize),
            price_points: [{ head: Link::new(), tail: Link::new() }; Price::max_value()+1],
            id: 1,
            execution_log: Vec::new(),
            should_log: false
        }
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
        let isask = is_ask(order.side);
        let book = if isask { &mut self.bids } else { &mut self.asks };
        let cross_test = if isask { Engine::hit_bid } else { Engine::hit_ask };
        let log = &mut self.execution_log;

        for matched_order in book.iter_mut() {
            if order.size == 0 {
                break;
            }
            if !cross_test(order.price, matched_order.order.price) {
                break;
            }

            Engine::trade(order, &mut matched_order.order, log, self.should_log);            
        }

        book.retain(|x| x.order.size > 0);

        order.size == 0
    }

    fn queue(&mut self, order: Order) {
        let isask = is_ask(order.side);
        let book = if isask { &mut self.asks } else { &mut self.bids };
        let cross_test = if isask { Engine::priority_ask } else { Engine::priority_bid };

        let insertion_index = match book.iter().enumerate().find(|(_index, ele)| cross_test(order.price, ele.order.price)) {
            Some((a, _)) => a,
            _ => book.len(),
        };
                            
        let new_order = OrderIn { order: order, id: self.id };
        book.insert(insertion_index, new_order);
    }

    pub fn limit_order(&mut self, mut order: Order) -> OrderId {
        // Cross off as many shares as possible.
        if is_ask(order.side) {
            if order.price >= self.ask_min {
                
            }
        }
        else {

        }

        5
    }

    pub fn cancel(&mut self, id: OrderId) {
        self.asks.retain(|x| x.id != id);
        self.bids.retain(|x| x.id != id);
    }

}