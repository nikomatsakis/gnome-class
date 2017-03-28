// gobject! {
//     class Counter {
//         let count: Cell<u32>;
//
//         new(f: u32) -> CounterFields {
//             CounterFields { count: Cell::new(f) }
//         }
//
//         fn add(this, a: u32) -> u32 {
//             let foo = this.Counter();
//             let v = foo.count.get() + a;
//             foo.count.set(v);
//             v
//         }
//
//         fn get(this) -> u32 {
//             this.Counter().count.get()
//         }
//     }
//
//     class MultCounter extends Counter {
//         let multiplier: u32;
//
//         new(m: u32) -> MultCounterFields {
//             let CounterFields = CounterFields::new(0);
//             MultCounterFields { CounterFields, multiplier: m }
//         }
//
//         fn multiplier(this) -> u32 {
//             this.MultCounter().multiplier
//         }
//
//         impl Counter {
//             fn add(this, a: u32) -> u32 {
//                  CounterSuper::add(this, a * this.MultCounter().multiplier)
//             }
//
//             fn get(this) {
//                  super // short for `CounterSuper::get(this)`
//             }
//         }
//     }
// }

use gobject::*;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use ptr::{Ptr, Weak};

///////////////////////////////////////////////////////////////////////////
// Counter

// eventually: #[repr(first)]
pub struct CounterFields {
    GObject: GObjectFields,
    count: Cell<u32>,
}

impl CounterFields {
    pub fn new(f: u32) -> CounterFields {
        // user code here
        CounterFields { GObject: GObjectFields::new(), count: Cell::new(f) }
    }
}

// eventually: #[repr(class)]
trait Counter: 'static {
    // ideally would be:
    // count: u32;

    // instead:
    fn Counter(&self) -> &CounterFields;

    fn add(&self, a: u32) -> u32;

    fn get(&self) -> u32;
}

trait CounterSuper {
    fn add(this: &Self, a: u32) -> u32;
    fn get(this: &Self) -> u32;
}

fn upcast_Counter<T: ?Sized + Counter>(p: &Ptr<T>) -> &Ptr<Counter> {
    panic!("FIXME")
}

impl<T: Counter> CounterSuper for Ptr<T> {
    fn add(this: &Self, a: u32) -> u32 {
        let this: Ptr<Counter> = this.clone();
        CounterSuper::add(&this, a)
    }

    fn get(this: &Self) -> u32 {
        let this: Ptr<Counter> = this.clone();
        CounterSuper::get(&this)
    }
}

impl CounterSuper for Ptr<Counter> {
    fn add(this: &Self, a: u32) -> u32 {
        let foo = this.Counter();
        let v = foo.count.get() + a;
        foo.count.set(v);
        v
    }

    fn get(this: &Self) -> u32 {
        this.Counter().count.get()
    }
}

impl Counter {
    pub fn new(f: u32) -> Ptr<Counter> {
        struct Impl {
            fields: CounterFields,
            self_ref: RefCell<Weak<Impl>>,
        }

        impl Counter for Impl {
            fn Counter(&self) -> &CounterFields {
                &self.fields
            }

            fn add(&self, a: u32) -> u32 {
                let self_ref = self.self_ref.borrow().upgrade().unwrap();
                CounterSuper::add(&self_ref, a)
            }

            fn get(&self) -> u32 {
                let self_ref = self.self_ref.borrow().upgrade().unwrap();
                CounterSuper::get(&self_ref)
            }
        }

        let ptr = Ptr::new(Impl {
            fields: CounterFields::new(f),
            self_ref: RefCell::new(Weak::new())
        });

        {
            let mut self_ref = ptr.self_ref.borrow_mut();
            *self_ref = Arc::downgrade(&ptr);
        }

        ptr
    }
}

///////////////////////////////////////////////////////////////////////////
// MultCounter
//
//pub struct MultCounterFields {
//    CounterFields: CounterFields,
//    multiplier: u32,
//}
//
//impl MultCounterFields {
//    pub fn new(m: u32) -> MultCounterFields {
//        let CounterFields = CounterFields::new(0);
//        MultCounterFields { CounterFields, multiplier: m }
//    }
//}
//
//trait MultCounter: Counter {
//    fn MultCounter(&self) -> &MultCounterFields;
//    fn multiplier(&self) -> u32;
//}
//
//trait MultCounterSuper {
//    fn multiplier(this: &Self) -> String;
//    fn add(this: &Self, a: u32) -> u32;
//    fn get(this: &Self) -> u32;
//}
//
//impl<T: ?Sized + MultCounter> MultCounterSuper for Ptr<T> {
//    fn multiplier(this: &Self) -> String {
//        fn m(this: &Ptr<MultCounter>) -> u32 {
//            this.MultCounter().multiplier
//        }
//
//        MultCounterSuper::add(this)
//    }
//
//    fn add(this: &Self, a: u32) -> u32 {
//        fn m(this: &Ptr<MultCounter>, a: u32) -> u32 {
//            CounterSuper::add(this, a * this.MultCounter().multiplier)
//        }
//    }
//
//    fn get(this: &Self) -> u32 {
//        CounterSuper::get(this)
//    }
//}
//
//impl MultCounter {
//    pub fn new(s: String) -> Ptr<MultCounter> {
//        struct Impl(MultCounterFields);
//
//        impl MultCounter for Ptr<Impl> {
//            fn MultCounter(&self) -> &MultCounterFields {
//                &self.0
//            }
//
//            fn multiplier(&self) -> u32 {
//                MultCounterSuper::multiplier(self)
//            }
//        }
//
//        impl Counter for Ptr<Impl> {
//            fn Counter(&self) -> &CounterFields {
//                &self.0.FooFields
//            }
//
//            fn add(&self, a: u32) -> u32 {
//                MultCounterSuper::add(self, a)
//            }
//
//            fn get(&self) -> u32 {
//                MultCounterSuper::add(self)
//            }
//        }
//
//        Ptr::new(Impl(MultCounterFields::new(s)))
//    }
//}
//
