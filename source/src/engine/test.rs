#[cfg(test)]

mod engine_tests {

    use crate::types::types::{Order, OrderId, Execution};
    use crate::engine::engine::{Engine, OrderIn};

    struct TestState {
        order_id: OrderId,
        engine: Engine,
    }

    impl TestState {

        fn new() -> TestState {
            TestState {
                order_id: 0,
                engine: Engine::new(),
            }
        }

        fn feed_orders(&mut self, orders: &mut Vec<Order>) {
            for mut order in orders {
                let id = self.engine.limit_order(&mut order);
                self.order_id += 1;

                assert_eq!(id, self.order_id);
            }
        }

        fn feed_cancels(&mut self, cancels: Vec<OrderIn>) {
            for cancel in cancels {
                self.engine.cancel(cancel);
            }
        }

        fn verify_exec_count(&self, expected_count: usize) {
            assert_eq!(self.engine.execution_log.len(), expected_count,
                        "Expected execution log size of {}, real was {}", expected_count, self.engine.execution_log.len());
        }

        // Pre condition: expected_log.len() == self.engine.execution_log.len()
        fn verify_exec_log(&self, expected_log: &Vec<Order>) {
            let len = expected_log.len();
            let mut index = 0;
            
            while index < len {

                let ordered_case = expected_log[index] == self.engine.execution_log[index] &&
                            expected_log[index+1] == self.engine.execution_log[index+1];
                let unordered_case = expected_log[index] == self.engine.execution_log[index+1] &&
                            expected_log[index+1] == self.engine.execution_log[index];

                assert_eq!(ordered_case || unordered_case, true, 
                    "Testing the equality of real {} & {} with expected {} & {}",
                    self.engine.execution_log[index], self.engine.execution_log[index+1],
                    expected_log[index], expected_log[index+1]);

                index += 2;
            }
        }

    }

    fn test(mut orders: Vec<Order>, execs: Vec<Execution>) {
        let mut state = TestState::new();

        state.feed_orders(&mut orders);
        state.verify_exec_count(execs.len());
        state.verify_exec_log(&execs);
    }

    fn test_cancel(mut orders_1: Vec<Order>, cancels: Vec<OrderIn>, mut orders_2: Vec<Order>, execs: Vec<Execution>) {

    }
 
    #[test]
    fn test_ask() {
        let oa101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 100};
        test(vec![oa101x100], vec![]);
    }

    #[test]
    fn test_bid() {
        let ob101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 100};
        test(vec![ob101x100], vec![]);
    }

    #[test]
    fn test_basic_exec() {
        let oa101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 100};
        let ob101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 100};

        let xa101x100: Execution = oa101x100.clone();
        let xb101x100: Execution = ob101x100.clone();

        test(vec![oa101x100, ob101x100], vec![xa101x100, xb101x100]);
    }

    #[test]
    fn test_partial_ask_fill() {
        let oa101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 100};
        let oa101x50: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 50};
        let ob101x50: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 50};

        let xa101x50: Execution = oa101x50.clone();
        let xb101x50: Execution = ob101x50.clone();

        test(vec![oa101x100, ob101x50], vec![xa101x50, xb101x50]);
    }

    #[test]
    fn test_partial_bid_fill() {
        let ob101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 100};
        let oa101x50: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 50};
        let ob101x50: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 50};

        let xa101x50: Execution = oa101x50.clone();
        let xb101x50: Execution = ob101x50.clone();

        test(vec![oa101x50, ob101x100], vec![xa101x50, xb101x50]);
    }

    #[test]
    fn test_increment_over_fill() {
        let oa101x100: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 100};
        let oa101x25: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 25};
        let ob101x25: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 25};

        let xa101x25: Execution = oa101x25.clone();
        let xb101x25: Execution = ob101x25.clone();

        test(vec![oa101x100, ob101x25.clone(), ob101x25.clone(), ob101x25.clone(), ob101x25.clone(), ob101x25.clone()], 
            vec![xa101x25.clone(), xb101x25.clone(), xa101x25.clone(), xb101x25.clone(), xa101x25.clone(), xb101x25.clone(), xa101x25.clone(), xb101x25.clone()]);
    }

    #[test]
    fn test_position() {
        let ob101x25x: Order = Order {symbol: String::from("JPM"), trader: String::from("BRETT"), side: false, price: 101, size: 25};
        let oa101x25: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: true, price: 101, size: 25};
        let ob101x25: Order = Order {symbol: String::from("JPM"), trader: String::from("MAX"), side: false, price: 101, size: 25};

        let xa101x25: Execution = oa101x25.clone();
        let xb101x25x: Execution = ob101x25x.clone();

        test(vec![ob101x25x, ob101x25, oa101x25], vec![xa101x25, xb101x25x])
    }
}