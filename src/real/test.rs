#![cfg(test)]

use super::*;

#[test]
fn create_counter() {
    let c = Counter::new(20);
    c.add(2);
    assert_eq!(c.get(), 22);
}
