#![feature(proc_macro)]

extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

use std::cell::Cell;

gobject_gen! {
    class One {
    }

    impl One {
        virtual fn get(&self) -> u32 {
            1
        }
    }
/*
    class Two: One {
    }

    impl One for Two {
        fn get(&self) -> u32 {
            2
        }
    }
*/
}


#[test]
fn test() {
    let one = One::new();
//    let two = Two::new();

    assert!(one.get() == 1);
//    assert!(two.get() == 2);
}
