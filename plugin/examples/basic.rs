#![feature(proc_macro)]

extern crate gobject_gen;

use gobject_gen::gobject_gen;

gobject_gen! {
    class Counter {
        struct CounterPrivate {
            f: Cell<u32>
        }
    }
}

fn main() {
    Dummy;
}
