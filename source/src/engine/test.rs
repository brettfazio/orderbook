#[cfg(test)]

mod engine_tests {

    use crate::types::types::{Order, OrderId, Execution};
    use crate::engine::engine::Engine;

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

        fn verify_exec_count(&self, expected_count: usize) {
            assert_eq!(self.engine.execution_log.len(), expected_count,
                        "Expected exec log size of {}, real was {}", expected_count, self.engine.execution_log.len());
        }

        fn verify_exec_log(&self, expected_log: &Vec<Order>) {
            for (real, expect) in self.engine.execution_log.iter().zip(expected_log.iter()) {
                assert_eq!(real == expect, true, "Testing the equality of real {} and expected {}", real, expect);
            }
        }

    }

    fn test(mut orders: Vec<Order>, execs: Vec<Execution>) {
        let mut state = TestState::new();

        state.feed_orders(&mut orders);
        state.verify_exec_count(execs.len());
        state.verify_exec_log(&execs);
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
}