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

use g::{self, G, GInstance, GClass, GSubclass};
use glib_sys::{GType, gpointer};
use gobject::*;
use gobject_sys::{self, GObjectClass, GTypeFlags, GTypeInstance};
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
#[repr(C)]
pub struct Counter {
    parent: GObject,
    // XXX I want some way to ensure that `Counter` is never created
    // except by `GObject_new`, basically some "unconstructable" type.
}

struct CounterPrivate {
    f: Cell<u32>,
    dc: Option<DropCounter>,
}

#[repr(C)]
pub struct CounterClass {
    parent_class: GObjectClass,
    add: extern fn(&Counter, v: u32) -> u32,
    get: extern fn(&Counter) -> u32,
}

unsafe impl GInstance for Counter {
    type Class = CounterClass;
}

unsafe impl GClass for CounterClass {
    type Instance = Counter;
}

unsafe impl GSubclass for CounterClass {
    type ParentClass = GObjectClass;
}

impl Counter {
    pub fn new() -> G<Counter> {
        unsafe {
            let data: *mut GObject =
                gobject_sys::g_object_new(*COUNTER_GTYPE, ptr::null_mut());
            G::new(data as *mut Counter)
        }
    }

    fn private(&self) -> &CounterPrivate {
        unsafe {
            let this = self as *const Counter as *mut GTypeInstance;
            let private = gobject_sys::g_type_instance_get_private(this, *COUNTER_GTYPE);
            let private = private as *const CounterPrivate;
            &*private
        }
    }

    pub fn to_ref(&self) -> G<Counter> {
        g::to_object_ref(self).clone()
    }

    pub fn upcast(&self) -> &GObject {
        &self.parent
    }

    pub fn add(&self, v: u32) -> u32 {
        println!("self={:p}", self);
        println!("get_class={:p}", g::get_class(self));
        (g::get_class(self).add)(self, v)
    }

    pub fn get(&self) -> u32 {
        (g::get_class(self).get)(self)
    }
}

impl CounterClass {
    extern "C" fn init(klass: gpointer, _klass_data: gpointer) {
        println!("klass={:?}", klass);

        unsafe {
            let g_object_class = klass as *mut GObjectClass;
            (*g_object_class).finalize = Some(Counter::finalize);

            let klass = klass as *mut CounterClass;
            let klass: &mut CounterClass = &mut *klass;
            klass.add = methods::add;
            klass.get = methods::get;
        }
    }
}

mod methods {
    #[allow(unused_imports)]
    use super::{Counter, CounterPrivate, CounterClass};

    pub(super) extern fn add(this: &Counter, v: u32) -> u32 {
        let private = this.private();
        let v = private.f.get() + v;
        private.f.set(v);
        v
    }

    pub(super) extern fn get(this: &Counter) -> u32 {
        this.private().f.get()
    }
}

impl Counter {
    extern "C" fn init(this: *mut GTypeInstance, _klass: gpointer) {
        fn new() -> CounterPrivate {
            CounterPrivate { f: Cell::new(0), dc: None }
        }

        unsafe {
            let private = gobject_sys::g_type_instance_get_private(this, *COUNTER_GTYPE);
            let private = private as *mut CounterPrivate;
            ptr::write(private, new());
        }
    }

    extern "C" fn finalize(this: *mut gobject_sys::GObject) {
        let this = this as *mut Counter;
        unsafe {
            ptr::read((*this).private());

            let object_class = g::get_class(&*this);
            let parent_class = g::get_parent_class(object_class);
            if let Some(f) = parent_class.finalize {
                f(this as *mut gobject_sys::GObject);
            }
        }
    }
}

lazy_static! {
    pub static ref COUNTER_GTYPE: GType = {
        unsafe {
            gobject_sys::g_type_register_static_simple(
                gobject_sys::g_object_get_type(),
                b"Counter\0" as *const u8 as *const i8,
                mem::size_of::<CounterClass>() as u32,
                Some(CounterClass::init),
                mem::size_of::<Counter>() as u32,
                Some(Counter::init),
                GTypeFlags::empty())
        }
    };
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
