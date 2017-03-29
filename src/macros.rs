macro_rules! __gobject__ {
    (
        class ($Class:ident, $ClassFields:ident, $ClassPtr:ident, $ClassSuper:ident) {
            // "fields" -- the list of fields
            fields {
                $(($($fpub:tt)*) $fname:ident: $fty:ty),*
            }

            // "new" -- the constructor
            new($($narg:ident: $narg_ty:ty),*) { $($nret_body:tt)* }

            // "methods" -- the list of methods introduced by this class
            methods {
                $(fn $mname:ident($mthis:ident, $($marg:ident:$marg_ty:ty),*) -> $mret_ty:ty {
                    $($mbody:tt)*
                })*
            }

            // "impls" -- impls of the superclass; eventually we should figure out how
            // to supper nesting > 1
            extends ($SClass:ident, $SClassFields:ident, $SClassPtr:ident) {
                $(fn $sname:ident($sthis:ident, $($sarg:ident:$sarg_ty:ty),*) -> $sret_ty:ty {
                    $($sbody:tt)*
                })*
            }
        }
    ) => {
        // Generate the struct `ClassFields` that houses the fields of this class.
        //
        // e.g.:
        //
        // struct CounterFields {
        //     GObject: GObjectFields,
        //     count: Cell<u32>,
        // }
        struct $ClassFields {
            $SClass: $SClassFields,
            $($($fpub)* $fname: $fty,)*
        }

        // Generate the constructor code that yields up the fields;
        // this is only used for `super` calls, effectively, that appear
        // in a constructor.
        //
        // e.g.:
        //
        // impl CounterFields {
        //   pub fn new(f: u32) -> CounterFields {
        //     CounterFields {
        //         GObject: GObjectFields::new(),
        //         count: Cell::new(f),
        //     }
        //   }
        // }
        impl $ClassFields {
            pub fn new($($narg: $narg_ty),*) -> Self {
                $($nret_body)*
            }
        }

        // Generate the main trait `Class` that serves for references
        // to this class.
        //
        // e.g.
        //
        // trait Counter {
        //     fn Counter(&self) -> &CounterFields;
        //     fn CounterPtr(&self) -> Ptr<Counter>;
        //     fn add(&self, a: u32) -> u32;
        //     fn get(&self) -> u32;
        // }
        trait $Class: $SClass {
            fn $Class(&self) -> &$ClassFields;
            fn $ClassPtr(&self) -> Ptr<$Class>;

            $(
                fn $mname(&self, $($marg: $marg_ty,)*) -> $mret_ty;
            )*
        }

        // Generate the `ClassSuper` trait that houses the code for
        // this class.  This contains **both** the code for methods
        // added by this class and for methods overridden from the
        // superclass.
        //
        // NB: These are specifically defined in "free fn" form.
        trait $ClassSuper {
            $(fn $mname($mthis: &Self, $($marg:$marg_ty,)*) -> $mret_ty;)*
            $(fn $sname($sthis: &Self, $($sarg:$sarg_ty,)*) -> $sret_ty;)*
        }

        // Impl the `ClassSuper` trait for any `Ptr<This>` where `This: Class`.
        impl<This: ?Sized + $Class> $ClassSuper for Ptr<This> {
            $(
                fn $mname($mthis: &Self, $($marg:$marg_ty,)*) -> $mret_ty {
                    fn m($mthis: &Ptr<$Class>, $($marg:$marg_ty,)*) -> $mret_ty {
                        $($mbody)*
                    }

                    m(&$mthis.$ClassPtr(), $($marg,)*)
                }
            )*

            $(
                fn $sname($sthis: &Self, $($sarg:$sarg_ty),*) -> $sret_ty {
                    fn m($sthis: &Ptr<$Class>, $($sarg:$sarg_ty),*) -> $sret_ty {
                        $($sbody)*
                    }

                    m(&$sthis.$ClassPtr(), $($sarg),*)
                }
            )*
        }

        // Add a `new` method for creating instances of this class.
        impl $Class {
            pub fn new($($narg: $narg_ty),*) -> Ptr<$Class> {
                use std::cell::RefCell;
                use std::sync::{Arc, Weak};

                struct Impl {
                    fields: $ClassFields,
                    self_ref: RefCell<Weak<Impl>>,
                }

                impl $Class for Impl {
                    fn $Class(&self) -> &$ClassFields {
                        &self.fields
                    }

                    fn $ClassPtr(&self) -> Ptr<$Class> {
                        self.self_ref.borrow().upgrade().unwrap()
                    }

                    $(
                        fn $mname(&self, $($marg: $marg_ty),*) -> $mret_ty {
                            $ClassSuper::$mname(&self.$ClassPtr(), $($marg),*)
                        }
                    )*
                }

                impl $SClass for Impl {
                    fn $SClass(&self) -> &$SClassFields {
                        &self.fields.$SClass
                    }

                    fn $SClassPtr(&self) -> Ptr<$SClass> {
                        self.self_ref.borrow().upgrade().unwrap()
                    }

                    $(
                        fn $sname(&self, $($sarg: $sarg_ty),*) -> $sret_ty {
                            $ClassSuper::$sname(&self.$ClassPtr(), $($sarg),*)
                        }
                    )*
                }

                let ptr = Ptr::new(Impl {
                    fields: $ClassFields::new($($narg),*),
                    self_ref: RefCell::new(Weak::new())
                });

                let weak = Arc::downgrade(&ptr);
                *ptr.self_ref.borrow_mut() = weak;

                ptr
            }
        }
    }
}
