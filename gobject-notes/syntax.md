```rust
struct FooPrivate {
    ...
}

// or #[derive(Default)] above if it works for you
impl Default for FooPrivate {
    fn default() -> FooPrivate {
        ...
    }
}

struct FooClassPrivate {
    ...
}

gnome_class! {
    class Foo: Superclass {
        type InstancePrivate = FooPrivate; // similar to associated types, "type Foo = Bar;"
        type ClassPrivate = FooClassPrivate;
    }

    // this defines the class ABI, basically
    impl Foo {
        pub fn static_method(&self, ...) {
            ...
        }

        virtual fn virtual_method(&self, ...) {
            ...
        }

        fn this_private_method_is_an_implementation_detail(&self) {
            // and is not exported or put in the class slots
        }

        signal some_signal(&self, ...);

        signal with_default_handler(&self, ...) -> Bar {
            // default handler code goes here
        }

        #[accumulator(my_accumulator_function)]
        signal sig_with_accumulator(&self, ...);

        reserve_slots(5); // decrement this when you add a method/signal, for ABI compatibility
    }

    // from sig_with_accumulator above
    fn my_accumulator(/* FIXME: arguments */) -> bool {
        ...
    }

    // Properties.  These could go in the "impl Foo" above?
    // See https://wiki.gnome.org/Projects/Vala/Manual/Classes#Properties for ideas
    impl Foo {
        #[attributes...]
        property some_property: T where T: u32 {
            get(&self) -> T {
                ...
            }

            set(&self, value: T) {
                ...
            }
        }

        #[construct]
        // #[construct_only]
        property foo: T where T: u32 {
            default() -> T {
                // warn if a non-construct property has a default() as it won't be used?
                // require construct or construct-only properties to have a default()?
                ... 
            }

            get(&self) -> T {
                ...
            }

            set(&self, value: T) {
                ...
            }
        }
    }

    impl Superclass for Foo {
        fn overriden_method(&self, ...) {
            ...
        }

        signal overriden_signal_handler(&self, ...) {
            ...
        }
    }

    impl AnotherSuperclass for Foo {
        // same as above
    }

    // See https://wiki.gnome.org/Projects/Vala/Manual/Classes#Construction for syntax ideas

    // This "impl GObject" is an alternative to putting
    // initialization/destruction functions inside the "class" block.
    impl GObject for Foo {
        fn init(&self) {
            // set up initial things
        }

        #[construct_prop(name="foo-bar", arg="foobar")]
        #[construct_prop(name="eek", arg="eek")]
        fn constructor(&self, foobar: i32, eek: Eek) {
        }

        fn dispose(&self) {
        }
    }

    // should we list SomeInterface in the "class" line above?
    // Pros: makes it obvious at a glance what interfaces are implemented
    // Cons: a little duplication
    impl SomeInterface for Foo {
        fn blah(&self, ...) {
        }
    }
}
```
