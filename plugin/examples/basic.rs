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

        fn add(this, x: u32) -> u32 {
            let private = this.private();
            let v = private.f.get() + v;
            private.f.set(v);
            v
        }

        fn get(this) -> u32 {
            this.private().f.get()
        }
    }
}

fn main() {
}
