#![feature(proc_macro)]

extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

use std::cell::Cell;

struct CounterPrivate {
    f: Cell<u32>
}

impl Default for CounterPrivate {
    fn default() -> Self {
        CounterPrivate {
            f: Cell::new(0)
        }
    }
}

gobject_gen! {
    class Counter {
        type InstancePrivate = CounterPrivate;
    }

    impl Counter {
        pub fn add(&self, x: u32) -> u32 {
            let private = self.get_priv();
            let v = private.f.get() + x;
            private.f.set(v);
            v
        }

        pub fn get(&self) -> u32 {
            self.get_priv().f.get()
        }
    }
}

#[test]
fn test() {
    let c: Counter = Counter::new();

    println!("Counter has value: {}", c.get());

    c.add(2);
    c.add(20);
    assert_eq!(c.get(), 22);

    println!("Counter has value: {}", c.get());
}
