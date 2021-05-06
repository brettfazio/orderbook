

pub mod types {
    use std::fmt;

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

    impl fmt::Display for Order {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(${} from {}. {} {}x{})", self.symbol, self.trader, if self.side {"ask"} else {"bid"}, self.price, self.size)
        }
    }

    pub type Execution = Order;
}