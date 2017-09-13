#![feature(proc_macro)]

// FIXME: can we combine these two?
extern crate gnome_class_shims;
extern crate gobject_gen;

#[macro_use]
extern crate glib;
use gobject_gen::gobject_gen;

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct DropCounter {
    counter: Arc<AtomicUsize>
}

impl DropCounter {
    pub fn new() -> Self {
        DropCounter { counter: Arc::new(AtomicUsize::new(0)) }
    }

    pub fn get(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

gobject_gen! {
    class Dummy {
        struct DummyPrivate {
            dc: RefCell<DropCounter>
        }

        init {
            DummyPrivate {
                dc: RefCell::new(DropCounter::new())
            }
        }

        fn set_dc(&self, dc: DropCounter) {
            let mut self_dc = self.private().dc.borrow_mut();
            *self_dc = dc;
        }
    }
}

#[test]
fn check() {
    let dc = DropCounter::new();

    {
        let c: Dummy = Dummy::new();
        c.set_dc(dc.clone());
        println!("Drop counter has value: {}", dc.get());
        assert_eq!(dc.get(), 0);
    }

    println!("Drop counter has value: {}", dc.get());
    assert_eq!(dc.get(), 1);
}
