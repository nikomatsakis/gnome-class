#![feature(proc_macro)]

extern crate gobject_gen;
use gobject_gen::gobject_gen;

gobject_gen! {
    class Counter {
        struct CounterPrivate {
            f: Cell<u32>
        }

        init() -> CounterPrivate {
            CounterPrivate { f: Cell::new(0) }
        }
    }
}

fn main() {
    println!("{}", XXX);
}
