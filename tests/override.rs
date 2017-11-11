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

        virtual fn get(&self) -> u32 {
            1
        }
    }

    class Two: One {
    }
/*

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
    let two = Two::new();

    assert!(one.one() == 1);
    assert!(one.get() == 1);
    assert!(two.one() == 1);
    assert!(two.get() == 1);
//    assert!(two.get() == 2);
}
