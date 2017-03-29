use gobject::*;
use std::cell::Cell;
use ptr::Ptr;

__gobject__! {
    class (Counter, CounterFields, CounterPtr, CounterSuper) {
        fields {
            () count: Cell<u32>;
        }

        new(c: u32) {
            CounterFields { GObject: GObjectFields::new(), count: Cell::new(c) }
        }

        methods {
            fn add(this, a: u32) -> u32 {
                let fields = this.Counter();
                let v = fields.count.get() + a;
                fields.count.set(v);
                v
            }

            fn get(this, ) -> u32 {
                this.Counter().count.get()
            }
        }

        extends (GObject, GObjectFields, GObjectPtr) {
        }
    }
}

__gobject__! {
    class (MultCounter, MultCounterFields, MultCounterPtr, MultCounterSuper) {
        fields {
            () mult: u32;
        }

        new(mult: u32) {
            let Counter = CounterFields::new(0);
            MultCounterFields { Counter, mult }
        }

        methods {
            fn multiplier(this, ) -> u32 {
                this.MultCounter().mult
            }
        }

        extends (Counter, CounterFields, CounterPtr) {
            extends (GObject, GObjectFields, GObjectPtr) {
            }

            fn add(this, a: u32) -> u32 {
                let m = this.MultCounter().mult;
                CounterSuper::add(this, a * m)
            }

            fn get(this, ) -> u32 {
                CounterSuper::get(this)
            }
        }
    }
}

mod test;
