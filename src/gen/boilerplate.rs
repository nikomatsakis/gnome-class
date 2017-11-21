use quote::{Tokens};

use super::*;
use self::cstringident::*;

// This has all the one-time boilerplate for a GObject implementation:
// the instance and class structs, the get_type(), instance_init(),
// class_init() functions, etc.
//
// Things which are a variable number of items (methods, signals,
// properties) are generated in separate modules, and are then
// included into this boilerplate.

impl<'ast> ClassContext<'ast> {
    pub fn gen_boilerplate(&self) -> Tokens {
        let ModuleName                       = &self.ModuleName;
        let ClassName                        = self.ClassName;
        let InstanceName                     = self.InstanceName;
        let InstanceNameFfi                  = self.InstanceNameFfi;
        let InstanceExt                      = &self.InstanceExt;
        let ParentClassFfi                   = self.ParentClassFfi;
        let ParentInstance                   = self.ParentInstance;
        let ParentInstanceFfi                = self.ParentInstanceFfi;
        let PrivateClassName                 = &self.PrivateClassName;

        let callback_guard                   = glib_callback_guard();
        let register_instance_private        = self.register_instance_private();
        let get_priv_fn                      = self.get_priv_fn();
        let init_priv_with_default           = self.init_priv_with_default();
        let free_instance_private            = self.free_instance_private();
        let get_type_fn_name                 = self.instance_get_type_fn_name();
        let imp_new_fn_name                  = self.imp_new_fn_name();

        let slots                            = self.slots();
        // let signal_id_names                  = self.signal_id_names();
        let slot_default_handlers            = self.imp_slot_default_handlers();
        let slot_assignments                 = self.slot_assignments();
        let signal_declarations              = self.signal_declarations();

        let instance_method_trampolines      = self.instance_method_trampolines();
        let instance_signal_trampolines      = self.instance_signal_trampolines();
        let instance_method_impls            = self.instance_method_impls();
        let instance_default_signal_handlers = self.instance_default_signal_handlers();
        let instance_name_string             = CStringIdent(*InstanceName);

        let imp_extern_methods               = self.imp_extern_methods();

        let method_trait_fns                 = &self.method_trait_fns();
        let method_redirects                 = self.method_redirects();
        let signal_trampolines               = self.signal_trampolines();

        let parent_instance_tokens = if self.class.gobject_parent {
            quote_cs! {}
        } else {
            quote_cs! { : #ParentInstance }
        };

        quote_cs! {
            pub mod #ModuleName {
                #![allow(non_snake_case)] // "oddly" named module above
                extern crate glib_sys as glib_ffi;
                extern crate gobject_sys as gobject_ffi;

                // #[macro_use]
                extern crate glib;

                // extern crate libc;

                use glib::IsA;
                // use glib::object::Downcast;
                // use glib::signal::connect;
                use glib::translate::*;
                use std::ptr;
                use std::mem;
                // use std::mem::transmute;

                // Bring in our parent's stuff so the user's implementation
                // can use what they had already defined there.
                // use super::*;

                // #[cfg(feature = "bindings")]
                // mod ffi;

                // #[cfg(feature = "bindings")]
                // pub mod imp {
                //     pub use ffi::*;
                // }

                glib_wrapper! {
                    pub struct #InstanceName(Object<imp::#InstanceNameFfi, imp::#ClassName>)
                        #parent_instance_tokens;

                    match fn {
                        get_type => || imp::#get_type_fn_name(),
                    }
                }

                pub mod imp {
                    // Bring in our grandparent's stuff so the user's
                    // implementation can use what they had already defined
                    // there. Note that this isn't guaranteed to get used though
                    // so stick an #[allow] on it
                    #[allow(unused_imports)]
                    use super::super::*;

                    use super::glib;
                    use super::glib_ffi;
                    use super::gobject_ffi;
                    // use super::libc;

                    use std::mem;
                    use std::ptr;

                    use glib::translate::*;

                    #[repr(C)]
                    pub struct #InstanceNameFfi {
                        pub parent: #ParentInstanceFfi,
                    }

                    #[repr(C)]
                    pub struct #ClassName {
                        pub parent_class: #ParentClassFfi,
                        #(#slots)*
                    }

                    // #[repr(u32)]
                    // enum Properties {
                    //     FIXMEDummy = 1,
                    //     // first one starts at 1
                    //     // FIXME - do not emit this enum at all if there are no properties
                    // }

                    // #[repr(u32)]
                    // enum Signals {
                    //     FIXMEDummy = 0,
                    //     // first one starts at 0
                    //     // FIXME - do not emit this enum at all if there are no signals
                    //     // #(#signal_id_names),*
                    // }

                    struct #PrivateClassName {
                        parent_class: *const #ParentClassFfi,
                        // properties:   *const Vec<*const gobject_ffi::GParamSpec>,
                        // signals:      *const Vec<u32>
                    }

                    static mut PRIV: #PrivateClassName = #PrivateClassName {
                        // we use this instead of "ptr::null()" because using
                        // function calls to set constants is feature-gated.
                        parent_class: 0 as *const _,
                        // properties:   0 as *const _,
                        // signals:      0 as *const _,
                    };

                    // We are inside the "mod imp".  We will create function
                    // implementations for the default handlers for methods and
                    // signals as "impl super::Foo { ... }", so that the &self in
                    // those functions will refer to the Rust wrapper object that
                    // corresponds to the GObject-ABI structs within "mod imp".
                    impl super::#InstanceName {
                        #get_priv_fn

                        #(#slot_default_handlers)*
                    }

                    impl #InstanceNameFfi {
                        #[allow(dead_code)] // not used if no virtual methods
                        fn get_class(&self) -> &#ClassName {
                            unsafe {
                                let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                                &*(klass as *const #ClassName)
                            }
                        }

                        // Instance struct and private data initialization, called from GObject
                        unsafe extern "C" fn init(obj: *mut gobject_ffi::GTypeInstance, _klass: glib_ffi::gpointer) {
                            #[allow(unused_variables)] // not used if no private
                            let obj = obj;
                            #callback_guard

                            #init_priv_with_default
                        }

                        unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
                            #callback_guard

                            #free_instance_private

                            (*(PRIV.parent_class as *mut gobject_ffi::GObjectClass)).finalize.map(|f| f(obj));
                        }

                        // FIXME: set_property() handler

                        // FIXME: get_property() handler

                        #(#instance_method_trampolines)*

                        #(#instance_signal_trampolines)*

                        #(#instance_method_impls)*

                        #(#instance_default_signal_handlers)*
                    }

                    impl #ClassName {
                        unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
                            #callback_guard

                            #register_instance_private

                            // GObjectClass methods; properties
                            {
                                let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
                                gobject_class.finalize = Some(#InstanceNameFfi::finalize);
                                // FIXME: gobject_class.set_property = Some(#InstanceNameFfi::set_property);
                                // FIXME: gobject_class.get_property = Some(#InstanceNameFfi::get_property);

                                // FIXME
                                // let mut properties = Vec::new();
                                //
                                // create each property

                            }

                            // Slots
                            {
                                #[allow(unused_variables)] // not used if no virtual methods
                                let klass = &mut *(klass as *mut #ClassName);
                                #(#slot_assignments)*
                            }

                            // Signals
                            {
                                #(#signal_declarations)*
                            }

                            PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as *const #ParentClassFfi;
                        }
                    }

                    #[no_mangle]
                    pub unsafe extern "C" fn #imp_new_fn_name(/* FIXME: args */) -> *mut #InstanceNameFfi {
                        #callback_guard

                        let this = gobject_ffi::g_object_newv(
                            #get_type_fn_name(),
                            0,              // FIXME: num_arguments
                            ptr::null_mut() // FIXME: args
                        );

                        this as *mut #InstanceNameFfi
                    }

                    #(#imp_extern_methods)*

                    #[no_mangle]
                    pub unsafe extern "C" fn #get_type_fn_name() -> glib_ffi::GType {
                        #callback_guard

                        use std::sync::{Once, ONCE_INIT};
                        use std::u16;

                        static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
                        static ONCE: Once = ONCE_INIT;

                        ONCE.call_once(|| {
                            let class_size = mem::size_of::<#ClassName>();
                            assert!(class_size <= u16::MAX as usize);

                            let instance_size = mem::size_of::<#InstanceNameFfi>();
                            assert!(instance_size <= u16::MAX as usize);

                            TYPE = gobject_ffi::g_type_register_static_simple(
                                <#ParentInstance as glib::StaticType>::static_type().to_glib(),
                                #instance_name_string as *const u8 as *const i8,
                                class_size as u32,
                                Some(#ClassName::init),
                                instance_size as u32,
                                Some(#InstanceNameFfi::init),
                                gobject_ffi::GTypeFlags::empty()
                            );

                            // FIXME: add interfaces
                        });

                        TYPE
                    }

                }

                impl #InstanceName {
                    // FIXME: we should take construct-only arguments and other convenient args to new()
                    pub fn new() -> #InstanceName {
                        unsafe { from_glib_full(imp::#imp_new_fn_name(/* FIXME: args */)) }
                    }
                }

                pub trait #InstanceExt {
                    #(#method_trait_fns)*

                    // FIXME: property setters/getters like in glib-rs
                    //
                    // fn get_property_foo(&self) -> type;
                    //
                    // fn set_property_foo(&self, v: type);

                    // FIXME: methods to connect to signals like in glib-rs
                    //
                    // fn connect_signalname<F: Fn(&Self, type, type) -> type + 'static>(&self, f: F) -> u64;
                }

                impl<O: IsA<#InstanceName> + IsA<glib::object::Object>> #InstanceExt for O {
                    #(#method_redirects)*

                    // FIXME: property setters/getters like in glib-rs
                    //
                    // fn get_property_foo(&self) -> type {
                    //     let mut value = Value::from(&false); // FIXME: Value::from(&what)?
                    //     unsafe {
                    //         gobject_ffi:g_object_get_property(self.to_glib_none().0, "foo".to_glib_none().0, value.to_glib_none_mut().0);
                    //     }
                    //     value.get().unwrap()
                    // }
                    //
                    // fn set_property_foo(&self, v: type) {
                    //     unsafe {
                    //         gobject_ffi:g_object_set_property(self.to_glib_none().0, "foo".to_glib_none().0, Value::from(&v).to_glib_none().0);
                    //     }
                    // }

                    // FIXME: methods to connect to signals like in glib-rs
                    //
                    // fn connect_signalname<F: Fn(&Self, type, type) -> type + 'static>(&self, f: F) -> u64 {
                    //     unsafe {
                    //         let f: Box_<Box_<Fn(&Self, type, type) -> type + 'static>> = Box_::new(Box_::new(f));
                    //         connect(self.to_glib_none().0, "signalname",
                    //             transmute(signalname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
                    //     }
                    // }
                }

                #(#signal_trampolines)*
            }

            pub use #ModuleName::*;
        }
    }
}
