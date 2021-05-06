#[cfg(test)]

mod engine_tests {

    use crate::types::types::{Order, OrderId, Execution};
    use crate::engine::engine::Engine;

    /*
      orderid = 0;
  totaltests++;
  exec_overflow = 0;
  execs_out_iter = execs_out;
  execs_out_len = 0;
    */
    struct TestState {
        order_id: OrderId,
        executions: Vec<Execution>,
        engine: Engine,
    }

    impl TestState {

        pub fn execution(&self, exec: Execution) {

        }

        fn new() -> TestState {
            
            TestState {
                order_id: 0,
                executions: Vec::<Execution>::new(),
                engine: Engine::new(execution),
            }
        }

    }

    /*
    int test(t_order orders[], unsigned orders_len, t_execution execs[], unsigned execs_len)
{
  int ok = 1;
  set_globals();
  init();
  ok = ok && feed_orders(orders, orders_len);
  ok = ok && assert_exec_count(execs_len);
  ok = ok && assert_execs(execs, execs_len);
  destroy();
  if (!ok)
    printf("test %i failed.\n\n", totaltests);
  return ok;
}*/

    fn feed_orders(orders: Vec<Order>) {
        for order in orders {
            //let id = 
        }
    }

    fn test(orders: Vec<Order>, execs: Vec<Execution>) {

    }
 
    #[test]
    fn it_works() {
        
        assert_eq!(2 + 2, 4);
    }
}