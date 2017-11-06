#![feature(proc_macro)]

extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

use std::cell::Cell;

gobject_gen! {
    class Counter {
        struct CounterPrivate {
            f: Cell<u32>
        }

        private_init() -> CounterPrivate {
            CounterPrivate {
                f: Cell::new(0)
            }
        }
/*
        fn add(&self, x: u32) -> u32 {
            let private = self.get_priv();
            let v = private.f.get() + x;
            private.f.set(v);
            v
        }

        fn get(&self) -> u32 {
            self.get_priv().f.get()
        }
*/
    }
}

#[test]
#[cfg(None)]
fn test() {
    let c: Counter = Counter::new();

    println!("Counter has value: {}", c.get());

    c.add(2);
    c.add(20);
    assert_eq!(c.get(), 22);

    println!("Counter has value: {}", c.get());
}
