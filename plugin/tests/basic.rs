#![feature(proc_macro)]

// FIXME: remove the need for this
#[macro_use] extern crate lazy_static;

// FIXME: can we combine these two?
extern crate gnome_class_shims;
extern crate gobject_gen;
use gobject_gen::gobject_gen;

use gnome_class_shims::G;
use std::cell::Cell;

gobject_gen! {
    class Counter {
        struct CounterPrivate {
            f: u32
        }

        fn add(&self, x: u32) -> u32 {
            let mut private = self.private_mut();
            let v = private.f + x;
            private.f = v;
            v
        }

        fn get(&self) -> u32 {
            self.private().f
        }
    }
}

#[test]
fn test() {
    let c: G<Counter> = Counter::new();

    println!("Counter has value: {}", c.get());

    c.add(2);
    c.add(20);
    assert_eq!(c.get(), 22);

    println!("Counter has value: {}", c.get());
}
