#![feature(proc_macro)]

// FIXME: remove the need for this
#[macro_use] extern crate lazy_static;

// FIXME: can we combine these two?
extern crate gnome_class_shims;
extern crate gobject_gen;
use gobject_gen::gobject_gen;

use gnome_class_shims::G;
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
            dc: DropCounter
        }

        init {
            DummyPrivate {
                dc: DropCounter::new()
            }
        }

        fn set_dc(&self, dc: DropCounter) {
            let mut private = self.private_mut();
            private.dc = dc;
        }
    }
}

#[test]
fn check() {
    let dc = DropCounter::new();

    {
        let c: G<Dummy> = Dummy::new();
        c.set_dc(dc.clone());
        println!("Drop counter has value: {}", dc.get());
        assert_eq!(dc.get(), 0);
    }

    println!("Drop counter has value: {}", dc.get());
    assert_eq!(dc.get(), 1);
}
