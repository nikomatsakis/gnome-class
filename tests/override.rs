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
        pub fn one(&self) -> u32 {
            1
        }

        virtual fn get(&self, i: u32, j: u32) -> u32 {
            1 + i + j
        }
    }

    class Two: One {
    }

    impl One for Two {
        virtual fn get(&self, i: u32, j: u32) -> u32 {
            2 + i + j
        }
    }
}

#[test]
fn test() {
    let one = One::new();
    let two = Two::new();

    assert!(one.one() == 1);
    assert!(one.get(0, 0) == 1);
    assert!(two.one() == 1);
    assert!(two.get(0, 0) == 2);
}
