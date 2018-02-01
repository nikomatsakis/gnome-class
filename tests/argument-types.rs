#![deny(warnings)]
#![feature(proc_macro)]

extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

extern crate glib_sys;

gobject_gen! {
    class Foo {
    }

    impl Foo {
        pub fn method(&self) -> u32 {
            5
        }
    }
}

gobject_gen! {
    class Test {
    }

    impl Test {
        pub fn one(&self) -> bool {
            true
        }

        pub fn two(&self, a: bool, b: i32) -> bool {
            self.three(a) && b == b
        }

        fn three(&self, a: bool) -> bool {
            a
        }

        virtual fn four(&self, arg: bool) -> bool {
            self.three(arg)
        }

        pub fn five(&self) {
        }

        virtual fn six(&self) {
        }

        pub fn seven(&self, a: u32) {
            drop(a);
        }

        virtual fn eight(&self, b: usize) -> i8 {
            b as i8
        }

        virtual fn nine(&self, c: char) -> char {
            c
        }

        pub fn ten(&self, c: &Foo) -> bool {
            c.method() == self.method()
        }

        pub fn eleven(&self, c: &Test) -> bool {
            c.method() == self.method()
        }


        fn method(&self) -> u32 {
            3
        }
    }
}


#[test]
fn test() {
    use glib_sys::*;

    let t = Test::new();
    assert!(t.two(true, 2));
    assert!(!t.two(false, 2));
    assert!(t.four(true));
    assert!(!t.four(false));

    let f = Foo::new();
    assert!(!t.ten(&f));
    assert!(t.eleven(&t));

    type T = <Test as glib::wrapper::Wrapper>::GlibType;
    type F = <Foo as glib::wrapper::Wrapper>::GlibType;

    // assert that the generated functions have the right type
    let _: unsafe extern fn(*mut T) -> gboolean =
        TestMod::imp::test_one;
    let _: unsafe extern fn(*mut T, gboolean) -> gboolean =
        TestMod::imp::test_two;
    let _: unsafe extern fn(*mut T, gboolean) -> gboolean =
        TestMod::imp::test_four;
    let _: unsafe extern fn(*mut T) =
        TestMod::imp::test_five;
    let _: unsafe extern fn(*mut T) =
        TestMod::imp::test_six;
    let _: unsafe extern fn(*mut T, u32) =
        TestMod::imp::test_seven;
    let _: unsafe extern fn(*mut T, usize) -> i8 =
        TestMod::imp::test_eight;
    let _: unsafe extern fn(*mut T, u32) -> u32 =
        TestMod::imp::test_nine;
    let _: unsafe extern fn(*mut T, *mut F) -> gboolean =
        TestMod::imp::test_ten;
    let _: unsafe extern fn(*mut T, *mut T) -> gboolean =
        TestMod::imp::test_eleven;
}
