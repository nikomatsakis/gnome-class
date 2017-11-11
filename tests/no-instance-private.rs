#![deny(warnings)]
#![feature(proc_macro)]

extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

gobject_gen! {
    class One {
    }

    impl One {
        pub fn get(&self) -> u32 {
            1
        }
    }
}


#[test]
fn test() {
    let one = One::new();
    assert!(one.get() == 1);
}
