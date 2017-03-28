/*!

Example:

```notrust
gobject! {
    class Foo {
        let foo_field: u32;

        new(f: u32) -> FooFields {
            FooFields { foo_field: f }
        }

        fn method() {
        }
    }

    class Bar extends Foo {
        pub let bar_field: String;

        new(s: String) -> BarFields {
            let FooFields = super(0);
            BarFields { FooFields, bar_field: s }
        }

        // Ideally you'd be able to "tighten these gradually".
        // Also, it'll be a pain for us to support more than one
        // level of inheritance to start.
        impl Foo {
            fn method(&self) {
            }
        }
    }
}
```

generates:

```notrust
// eventually: #[repr(first)]
pub struct FooFields {
    GObject: GObject,
    foo_field: u32,
}

// eventually: #[repr(first)]
pub struct BarFields {
    FooFields: FooFields,
    bar_field: String,
}

// eventually: #[repr(class)]
trait Foo {
    // ideally would be:
    // foo_field: u32;

    // instead:
    fn Foo(&self) -> &FooFields;

    // automatically added:
    fn FooPtr(&self) -> Ptr<Foo>;

    fn method(&self);
}

impl Foo {
    fn new(f: u32) -> Ptr<Foo> {
        struct FooImpl {
            fields: FooFields
        }

        impl Foo for Ptr<FooImpl> {
            fn Foo(&self) -> &FooFields {
                &self.fields
            }

            fn FooPtr(&self) -> Ptr<Foo> {
                self.clone()
            }
        }

        Ptr::new(FooImpl {
            // from user:
            FooFields { foo_field: f }
        })
    }
}

// eventually: #[repr(class)]
trait Bar: Foo {
    // ideally would be:
    // bar_pub: u32;

    fn Bar(&self) -> &BarFields;
    fn BarPtr(&self) -> Ptr<Bar>;
}

impl Bar {
    fn new(f: u32) -> Ptr<Foo> {
        struct BarImpl {
            fields: BarFields
        }

        impl Bar for Ptr<BarImpl> {
            fn Foo(&self) -> &FooFields {
                &self.fields
            }

            fn FooPtr(&self) -> Ptr<Foo> {
                self.clone()
            }
        }

        Ptr::new(FooImpl {
            // from user:
            FooFields { foo_field: f }
        })
    }
}

// ideally would be a `default` impl,
// but for now we won't have that option
impl<This: Bar> Foo for Ptr<This> {
    fn Foo(&self) -> &FooFields {
        &self.Bar().FooFields
    }

    fn FooPtr(&self) -> Ptr<Foo> {
    }

    fn method(&self) {
        fn user(this: &Ptr<Foo>, 
    }
}
```

Some open questions and obstacles:

- Should we support `&mut self` in any way?
  - Unclear that this would be useful.
  - Probably better to just teach people to use `Cell` and `RefCell`
- Should we handle mutability better (this is orthogonal, really)
- How to partition the *trait* vs the `Ptr` wrapper?
  - IOW, it might be nice for `Foo` to be a `struct Foo(Ptr<FooTrait>)`
- Upcasting

How to support **Clone** and interface around pointers?

- Would be nice to support `Ptr<Self>` methods
- Would be nice to permit `self` in a free function
- If we want this to work most like OOP, in the default methods we
  want `self` to have the type `Ptr<Foo>`; can this be achieved? Well,
  certainly it should be possible. That has some advantages. For
  example, if we `impl<T: ?Sized> Clone for Ptr<T>`, then one can just
  `clone` the `Ptr<Foo>` no problem.

*/
