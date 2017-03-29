// gobject! {
//     class Counter {
//         let count: Cell<u32>;
//
//         new(f: u32) -> CounterFields {
//             CounterFields { count: Cell::new(f) }
//         }
//
//         fn add(this, a: u32) -> u32 { // this: &Ptr<Counter>
//             let foo = this.Counter(); // foo: &CounterFields
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

use g::{self, G, GObjectContents};
use glib_sys::{GType, gpointer};
use gobject::*;
use gobject_sys::{self, GObject, GObjectClass, GTypeClass, GTypeFlags, GTypeInstance};
use std::cell::Cell;
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

//

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

///////////////////////////////////////////////////////////////////////////
// Counter

// eventually: #[repr(first)]
pub struct CounterFields {
    GObject: GObjectFields,
    count: Cell<u32>,
    finalized: DropCounter,
}

impl CounterFields {
    pub fn new(f: u32, i: DropCounter) -> CounterFields {
        // user code here
        CounterFields {
            GObject: GObjectFields::new(),
            count: Cell::new(f),
            finalized: i,
        }
    }
}

// eventually: #[repr(class)]
trait Counter: GObjectContents {
    // ideally would be:
    // count: u32;

    // instead:
    fn Counter(&self) -> &CounterFields;
    fn CounterG(&self) -> G<Counter>;

    fn add(&self, a: u32) -> u32;

    fn get(&self) -> u32;
}

trait CounterSuper {
    fn add(this: &Self, a: u32) -> u32;
    fn get(this: &Self) -> u32;
}

impl<T: ?Sized + Counter> CounterSuper for G<T> {
    fn add(this: &Self, a: u32) -> u32 {
        fn m(this: &G<Counter>, a: u32) -> u32 {
            let foo = this.Counter();
            let v = foo.count.get() + a;
            foo.count.set(v);
            v
        }
        m(&this.CounterG(), a)
    }

    fn get(this: &Self) -> u32 {
        fn m(this: &G<Counter>) -> u32 {
            this.Counter().count.get()
        }
        m(&this.CounterG())
    }
}

impl Counter {
    pub fn new(f: u32, dc: DropCounter) -> G<Counter> {
        struct Class {
            parent_class: GObjectClass
        }

        impl Class {
            extern "C" fn init(klass: gpointer, _klass_data: gpointer) {
                unsafe {
                    let g_object_class = klass as *mut GObjectClass;
                    (*g_object_class).finalize = Some(Instance::finalize);
                }
            }
        }

        struct Instance {
            fields: CounterFields,
        }

        unsafe impl GObjectContents for Instance {
        }

        impl Instance {
            extern "C" fn init(_this: *mut GTypeInstance, _klass: gpointer) { }

            extern "C" fn finalize(this: *mut GObject) {
                unsafe {
                    {
                        let this = &*(this as *mut Instance);
                        ptr::read(&this.fields.count);
                        ptr::read(&this.fields.finalized);
                    }

                    // I am a horrible monster and I pray for death:
                    // the first field of `GObject` has type `*mut
                    // GTypeClass`, but it is private in the
                    // `gobject_sys` crate. Therefore, we cast this
                    // pointer to a pointer to the first field and
                    // read from it.
                    let object_class = {
                        let xxx = this as *mut *mut GTypeClass;
                        *xxx
                    };

                    let parent_class = gobject_sys::g_type_class_peek_parent(object_class as gpointer);
                    let g_parent_class = parent_class as *mut GObjectClass;
                    if let Some(f) = (*g_parent_class).finalize {
                        f(this);
                    }
                }
            }
        }

        impl Counter for Instance {
            fn Counter(&self) -> &CounterFields {
                &self.fields
            }

            fn CounterG(&self) -> G<Counter> {
                g::to_ref(self)
            }

            fn add(&self, a: u32) -> u32 {
                CounterSuper::add(&self.CounterG(), a)
            }

            fn get(&self) -> u32 {
                CounterSuper::get(&self.CounterG())
            }
        }

        lazy_static! {
            pub static ref COUNTER_GTYPE: GType = {
                unsafe {
                    gobject_sys::g_type_register_static_simple(
                        gobject_sys::g_object_get_type(),
                        b"Counter\0" as *const u8 as *const i8,
                        mem::size_of::<Class>() as u32,
                        Some(Class::init),
                        mem::size_of::<Instance>() as u32,
                        Some(Instance::init),
                        GTypeFlags::empty())
                }
            };
        }

        unsafe {
            let fields = CounterFields::new(f, dc);
            let data: *mut GObject = gobject_sys::g_object_new(*COUNTER_GTYPE, ptr::null_mut());

            // Dirty, dirty hack. At this point, both `fields` and
            // `data` have type `Instance`, but they have different
            // parts initialized:
            //
            // ```
            // +-------------------+
            // | CounterFields     |
            // | +---------------+ |
            // | |             // ```
            //
            // In particular, in the `fields`, the `GObjectFields` at
            // the front are zeroed. In `data`, meanwhile, the gobject
            // fields are initialized, but the rest are not. We need
            // to copy over the initialized fields from `fields` into
            // `data`.
            //
            // You might be wondering why we do this. The goal is to
            // allow the user's constructor function to set their
            // fields to their final, initialized values in one shot
            // without having an intermediate state where they are
            // uninitalized (or set to default values, which in Rust
            // don't always exist).
            {
                let gobject_size = mem::size_of::<GObjectFields>();
                let fields_ptr = &fields as *const CounterFields as *const u8;
                let data_ptr = data as *mut u8;
                let copy_size = mem::size_of::<Instance>() - gobject_size;
                ptr::copy_nonoverlapping(fields_ptr.offset(gobject_size as isize),
                                         data_ptr.offset(gobject_size as isize),
                                         copy_size);
                mem::forget(fields);
            }

            G::new(data as *mut Instance)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
//// MultCounter
//
//pub struct MultCounterFields {
//    CounterFields: CounterFields,
//    multiplier: u32,
//}
//
//impl MultCounterFields {
//    pub fn new(m: u32) -> MultCounterFields {
//        let CounterFields = CounterFields::new(0);
//        MultCounterFields {
//            CounterFields,
//            multiplier: m,
//        }
//    }
//}
//
//trait MultCounter: Counter {
//    fn MultCounter(&self) -> &MultCounterFields;
//    fn MultCounterG(&self) -> G<MultCounter>;
//    fn multiplier(&self) -> u32;
//}
//
//trait MultCounterSuper {
//    fn multiplier(this: &Self) -> u32;
//    fn add(this: &Self, a: u32) -> u32;
//    fn get(this: &Self) -> u32;
//}
//
//impl<T: ?Sized + MultCounter> MultCounterSuper for G<T> {
//    fn multiplier(this: &Self) -> u32 {
//        fn m(this: &G<MultCounter>) -> u32 {
//            this.MultCounter().multiplier
//        }
//
//        m(&this.MultCounterG())
//    }
//
//    fn add(this: &Self, a: u32) -> u32 {
//        fn m(this: &G<MultCounter>, a: u32) -> u32 {
//            CounterSuper::add(this, a * this.MultCounter().multiplier)
//        }
//
//        m(&this.MultCounterG(), a)
//    }
//
//    fn get(this: &Self) -> u32 {
//        fn m(this: &G<MultCounter>) -> u32 {
//            CounterSuper::get(this)
//        }
//
//        m(&this.MultCounterG())
//    }
//}
//
//impl MultCounter {
//    pub fn new(m: u32) -> G<MultCounter> {
//        struct Impl {
//            fields: MultCounterFields,
//            self_ref: RefCell<Weak<Impl>>,
//        }
//
//        impl MultCounter for Impl {
//            fn MultCounter(&self) -> &MultCounterFields {
//                &self.fields
//            }
//
//            fn MultCounterG(&self) -> G<MultCounter> {
//                self.self_ref
//                    .borrow()
//                    .upgrade()
//                    .unwrap()
//            }
//
//            fn multiplier(&self) -> u32 {
//                MultCounterSuper::multiplier(&self.MultCounterG())
//            }
//        }
//
//        impl Counter for Impl {
//            fn Counter(&self) -> &CounterFields {
//                &self.fields.CounterFields
//            }
//
//            fn CounterG(&self) -> G<Counter> {
//                self.self_ref
//                    .borrow()
//                    .upgrade()
//                    .unwrap()
//            }
//
//            fn add(&self, a: u32) -> u32 {
//                MultCounterSuper::add(&self.MultCounterG(), a)
//            }
//
//            fn get(&self) -> u32 {
//                MultCounterSuper::get(&self.MultCounterG())
//            }
//        }
//
//        let ptr = G::new(Impl {
//                               fields: MultCounterFields::new(m),
//                               self_ref: RefCell::new(Weak::new()),
//                           });
//
//        {
//            let mut self_ref = ptr.self_ref.borrow_mut();
//            *self_ref = Arc::downgrade(&ptr);
//        }
//
//        ptr
//    }
//}

mod test;
