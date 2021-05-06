/*
t_orderid
    Order identification numbers which are returned by limit(). Used to uniquely identify the order for the sake of cancellation.

t_price
    Price of a share. In the range 0-65536, and interpreted as divided by 100. Eg the range is 000.00-655.36. Eg the price 123.45 = 12345; the price 23.45 = 2345; the price 23.4 = 2340

t_size
    Number of shares. 

t_side
    Boolean representing whether a bid (0) or ask (1).

t_order
    Order submitted to the matching engine containing symbol, trader name, side, price, and size.

t_execution
    Execution report sent to trader informing them of the symbol, side, price, and size of the transaction.
*/
pub mod types {
    pub type OrderId = u64;

    pub type Price = u16;

    pub type Size = u64;

    pub type Side = bool;
    pub fn is_ask(s: Side) -> bool { return s; }

    #[derive(Clone)]
    pub struct Order {
        pub symbol: String,
        pub trader: String,
        pub side: Side,
        pub price: Price,
        pub size: Size,
    }

    impl PartialEq for Order {
        fn eq(&self, other: &Self) -> bool {
            return self.symbol == other.symbol &&
                    self.trader == other.trader &&
                    self.side == other.side &&
                    self.price == other.price &&
                    self.size == other.size;
        }
    }

    impl Eq for Order { }

    pub type Execution = Order;
}