#![cfg(test)]

use super::*;

#[test]
fn create_counter() {
    // This `DropCounter` will help us track whether finalizer for `c` got run.
    let i = DropCounter::new();

    {
        let c: G<Counter> = Counter::new(20, i.clone());
        c.add(2);
        assert_eq!(c.get(), 22);
        assert_eq!(i.get(), 0); // not dropped yet
    }

    assert_eq!(i.get(), 1); // should be dropped now
}

//#[test]
//fn create_mult_counter() {
//    let c: Ptr<MultCounter> = MultCounter::new(2);
//    let d: Ptr<Counter> = c.CounterPtr();
//    d.add(2);
//    assert_eq!(c.get(), 4);
//}
