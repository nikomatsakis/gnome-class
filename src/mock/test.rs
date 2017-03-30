#![cfg(test)]

use super::*;

#[test]
fn create_counter() {
    // This `DropCounter` will help us track whether finalizer for `c` got run.
    let dc = DropCounter::new();

    {
        let c: G<Counter> = Counter::new();
        c.add(2);
        c.add(20);
        c.set_drop_counter(dc.clone());
        assert_eq!(c.get(), 22);
        assert_eq!(dc.get(), 0); // not dropped yet
    }

    assert_eq!(dc.get(), 1); // should be dropped now
}

//#[test]
//fn create_mult_counter() {
//    let c: Ptr<MultCounter> = MultCounter::new(2);
//    let d: Ptr<Counter> = c.CounterPtr();
//    d.add(2);
//    assert_eq!(c.get(), 4);
//}
