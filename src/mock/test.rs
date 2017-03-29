#![cfg(test)]

use super::*;

#[test]
fn create_counter() {
    let c: G<Counter> = Counter::new(20);
    c.add(2);
    assert_eq!(c.get(), 22);
}

//#[test]
//fn create_mult_counter() {
//    let c: Ptr<MultCounter> = MultCounter::new(2);
//    let d: Ptr<Counter> = c.CounterPtr();
//    d.add(2);
//    assert_eq!(c.get(), 4);
//}
