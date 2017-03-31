Example:

```
class Counter {
    struct CounterPrivate {
        f: Cell<u32>, dc: Option<DropCounter>
    }

    properties {
        int_prop ("age",
	          _("Age"),
		  _("Age of person in years"),
		  min_value,
		  max_value,
		  default_value,
		  flags | EXPLICIT_NOTIFY) {
            set(x: u32) {
	        println! ("extra code here");
		self.set_age (x); // this one does the notification
            }

            get() -> u32 {
	        println! ("extra code here");
	        self.age
            }
        },

        // this bit is from Vala
        [Description(nick = "age in years", blurb = "This is the person's age in years")]
        public int age { get; set; default = 32; }

        "hello-with-dashes": i32 where min_value = 0,
                                       max_value = 42,
                                       default = 10 => {
            set explicit_notify (value) {
                if self.field != value {
                    self.field = value;
                    notify_property_changed ("hello-with-dashes");
                        
                }
                self.field = value;
            }

            get () -> i32 {
                self.field
            }
        }

        string_prop ("name",
	             _("Name"),
		     _("Full name of person"),
		     nullable_default_value,
		     flags),

        enum_prop (GType for the enum), // FIXME, register an enum?

	flags_prop (GType for the flags), // FIXME, register the flags?

        param_prop (GType for the param), // FIXME: ?

        boxed_prop (GType for the param), // FIXME: ?

	value_array_prop (GParamSpec for the element spec),

        object_prop (GType for object),

        variant_prop (GVariantType, GVariant default_value),

        // look at gparamspecs.h for the possible types
    }

    init() -> CounterPrivate {
        CounterPrivate { f: Cell::new(0), dc: None }
    }
    
    fn add(this, v: u32) -> u32 {
        // Type of this: &Counter
        let private = this.private();
        let v = private.f.get() + v;
        private.f.set(v);
        v
    }
    
    fn get(this) -> u32 {
        this.private().f.get()
    }
}
```
