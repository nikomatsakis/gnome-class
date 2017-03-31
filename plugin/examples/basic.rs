#![feature(proc_macro)]

extern crate gobject_sys;
extern crate gobject_gen;
use gobject_gen::gobject_gen;

use std::cell::Cell;

gobject_gen! {
    class Counter {
        struct CounterPrivate {
            f: Cell<u32>
        }

        init {
            CounterPrivate { f: Cell::new(0) }
        }

        fn get(this, x: u32) { xxx }
    }
}

fn main() {
}
