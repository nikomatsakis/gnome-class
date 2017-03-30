#![feature(proc_macro)]

extern crate gobject_gen;

use gobject_gen::gobject_gen;

gobject_gen! {
    class Foo { }
}

fn main() { }
