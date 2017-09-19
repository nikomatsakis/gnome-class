// We give `ClassName` variables an identifier that uses upper-case.
#![allow(non_snake_case)]

use ast::*;
use errors::*;
use lalrpop_intern::{self, intern};
use quote::{Ident, Tokens, ToTokens};
use std::convert::Into;

macro_rules! quote_in {
    ($tokens:expr, $($t:tt)*) => {
        $tokens.append_all(Some(quote!{$($t)*}));
    }
}

// HYGIENE NOTE:
//
// I am using the `__` prefix to indicate names that, while visible
// to the user, are eventually intended to be hidden by hygiene.

pub fn classes(program: &Program) -> Result<Tokens> {
    let class_tokens =
        program.classes
               .iter()
               .map(|class| {
                   let cx = ClassContext::new(program, class)?;
                   cx.gen_class()
               })
               .collect::<Result<Vec<_>>>()?;
    Ok(quote! { #(#class_tokens)* })
}

fn signal_id_name(signal: &Signal) -> Identifier {
    Identifier {
        str: intern(&format!("{}_signal_id", signal.name.str))
    }
}

struct ClassContext<'ast> {
    program: &'ast Program,
    class: &'ast Class,
    private_struct: &'ast PrivateStruct,
    ModuleName: Identifier,
    FieldsName: Identifier,
    GClassName: Identifier,
    PrivateClassName: Identifier,
    MethodsFrom: Identifier,
    ParentInstance: Tokens,
    ParentInstanceFfi: Tokens,
    ParentClassFfi: Tokens,
    GObject: Tokens,
    GObjectFfi: Tokens,
    GObjectClassFfi: Tokens,
    InstanceExt: Identifier,
}

impl<'ast> ClassContext<'ast> {
    pub fn new(program: &'ast Program, class: &'ast Class) -> Result<Self> {
        let private_struct =
            class.members
                 .iter()
                 .filter_map(|member| match *member {
                     Member::PrivateStruct(ref ps) => Some(ps),
                     _ => None,
                 })
                 .next();

        let private_struct = match private_struct {
            Some(p) => p,
            None => bail!("no private struct found")
        };

        // If our class name is "Foo" and we have a suffix "Bar", generates a "FooBar" Identifier.
        // These are used for the generated module name, instance/class struct names, etc.
        macro_rules! container_name {
            ($class:expr, $suffix:expr) => {
                Identifier {
                    str: intern(&format!("{}{}", $class.name.str, $suffix))
                }
            };
        }

        let ModuleName       = container_name!(class, "Mod"); // toplevel "InstanceMod" module name
        let FieldsName       = container_name!(class, "Fields");
        let GClassName       = container_name!(class, "Class");
        let PrivateClassName = container_name!(class, "ClassPrivate");
        let InstanceExt      = container_name!(class, "Ext"); // public trait with all the class's methods

        let GObject = quote! { glib::Object };
        let GObjectFfi = quote! { gobject_ffi::GObject };
        let GObjectClassFfi = quote! { gobject_ffi::GObjectClass };

        // GObject is hardcoded in various places below
        let ParentInstance =
            class.extends
                 .as_ref()
                 .map(|c| c.ty())
                 .map(|c| quote! { #c })
                 .unwrap_or_else(|| GObject.clone());
        let ParentInstanceFfi =
            class.extends
                 .as_ref()
                 .map(|c| c.ty())
                 .map(|c| quote! { #c })
                 .unwrap_or_else(|| GObjectFfi.clone());
        let ParentClassFfi = quote! {
            <#ParentInstance as glib::wrapper::Wrapper>::GlibClassType
        };

        let InstanceName = class.name;
        let MethodsFrom = Identifier {
            str: intern(&format!("__methods_from_{}", InstanceName.str))
        };

        Ok(ClassContext {
            program,
            class,
            private_struct,
            ModuleName,
            FieldsName,
            GClassName,
            PrivateClassName,
            MethodsFrom,
            ParentInstance,
            ParentInstanceFfi,
            ParentClassFfi,
            GObject,
            GObjectFfi,
            GObjectClassFfi,
            InstanceExt,
        })
    }

    pub fn gen_class(&self) -> Result<Tokens> {
        let all = vec![
            self.imports(),
            self.glib_wrapper(),
            self.imp_module(),
            self.pub_impl(),
            self.instance_ext(),
            self.instance_ext_impl(),
//            self.impls(),
//            self.methods_declared_in_instance(),
//            self.signal_trampolines(),
//            self.c_symbols(),
        ];

        let ModuleName = &self.ModuleName;

        Ok(quote! {
            pub mod #ModuleName {
                #(#all)*
            }

            pub use #ModuleName::*;
        })
    }

    fn callback_guard(&self) -> Tokens {
        quote! {
            let _guard = glib::CallbackGuard::new();
        }
    }

    fn imports(&self) -> Tokens {
        quote! {
            extern crate glib_sys as glib_ffi;
            extern crate gobject_sys as gobject_ffi;

            // #[macro_use]
            extern crate glib;

            extern crate libc;

            use glib::{IsA, Value};
            use glib::object::Downcast;
            use glib::signal::connect;
            use glib::translate::*;

            use std::ptr;
            use std::mem;
            use std::mem::transmute;

            // Bring in our parent's stuff so the user's implementation
            // can use what they had already defined there.
            use super::*;

            // #[cfg(feature = "bindings")]
            // mod ffi;

            // #[cfg(feature = "bindings")]
            // pub mod imp {
            //     pub use ffi::*;
            // }
        }
    }

    fn exported_fn_name(&self, method_name: &str) -> Identifier {
        Identifier {
            str: intern(&format!("{}_{}", self.lower_case_class_name(), method_name))
        }
    }

    fn get_type_fn_name(&self) -> Identifier {
        self.exported_fn_name("get_type")
    }

    fn glib_wrapper(&self) -> Tokens {
        let InstanceName = self.class.name;
        let get_type_fn_name = self.get_type_fn_name();

        quote! {
            glib_wrapper! {
                pub struct #InstanceName(Object<imp::#InstanceName>); // FIXME: parent classes/interfaces

                match fn {
                    get_type => || imp::#get_type_fn_name(),
                }
            }
        }
    }

    fn imp_module(&self) -> Tokens {
        let all = vec![
            self.imp_instance_struct(),
            self.imp_class_struct(),
            self.imp_properties_enum(),
            self.imp_signals_enum(),
            self.imp_private_struct(),
            self.imp_class_private_struct(),
            self.imp_instance(),
            self.imp_class(),
            self.imp_extern_funcs(),
            self.imp_get_type_fn(),
        ];

        quote! {
            pub mod imp {
                // Bring in our grandparent's stuff so the user's implementation
                // can use what they had already defined there.
                use super::super::*;

                use super::glib;
                use super::glib_ffi;
                use super::gobject_ffi;
                use super::libc;

                use std::mem;
                use std::ptr;

                use glib::translate::*;

                #(#all)*
            }
        }
    }

    fn imp_instance_struct(&self) -> Tokens {
        let InstanceName = self.class.name;
        let ParentInstanceFfi = &self.ParentInstanceFfi;

        quote! {
            #[repr(C)]
            pub struct #InstanceName {
                pub parent: #ParentInstanceFfi,
            }
        }
    }

    fn imp_class_struct(&self) -> Tokens {
        let ClassName = self.GClassName;
        let ParentClassFfi = &self.ParentClassFfi;

        // ABI: we are generating the imp::FooClass with the parent_class, and the slots to signals/methods.
        // This defines the C ABI for the class structure.
        //
        // FIXME: we should check that the extern "C" signatures only have types representable by C.

        let slots: Vec<Tokens> = self.members_with_slots()
            .map(|member| {
                let (slot_name, slot_fn_ty) = match *member {
                    Member::Method(ref method) => (method.name,
                                                   SlotTy {
                                                       class_name: self.class.name,
                                                       sig: &method.fn_def.sig
                                                   }),

                    Member::Signal(ref signal) => (signal.name,
                                                   SlotTy {
                                                       class_name: self.class.name,
                                                       sig: &signal.sig
                                                   }),

                    _ => unreachable! ()
                };

                quote! {
                    pub #slot_name: Option<unsafe extern "C" fn#slot_fn_ty>,
                }
            })
            .collect();

        quote! {
            #[repr(C)]
            pub struct #ClassName {
                pub parent_class: #ParentClassFfi,
                #(#slots)*
            }
        }
    }

    fn imp_properties_enum(&self) -> Tokens {
        quote! {
            #[repr(u32)]
            enum Properties {
                FIXMEDummy = 1,
                // first one starts at 1
                // FIXME - do not emit this enum at all if there are no properties
            }
        }
    }

    fn imp_signals_enum(&self) -> Tokens {
        quote! {
            #[repr(u32)]
            enum Signals {
                FIXMEDummy = 0,
                // first one starts at 0
                // FIXME - do not emit this enum at all if there are no signals
            }
        }
    }

    fn imp_get_type_fn(&self) -> Tokens {
        let callback_guard = self.callback_guard();
        let get_type_fn_name = self.get_type_fn_name();
        let GClassName = self.GClassName;
        let InstanceName = self.class.name;
        let ParentInstance = &self.ParentInstance;
        let instance_name_string = ByteString(InstanceName);

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #get_type_fn_name() -> glib_ffi::GType {
                #callback_guard

                use std::sync::{Once, ONCE_INIT};
                use std::u16;

                static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
                static ONCE: Once = ONCE_INIT;

                ONCE.call_once(|| {
                    let class_size = mem::size_of::<#GClassName>();
                    assert!(class_size <= u16::MAX as usize);

                    let instance_size = mem::size_of::<#InstanceName>();
                    assert!(instance_size <= u16::MAX as usize);

                    TYPE = gobject_ffi::g_type_register_static_simple(
                        <#ParentInstance as glib::StaticType>::static_type().to_glib(),
                        #instance_name_string as *const u8 as *const i8,
                        class_size as u32,
                        Some(#GClassName::init),
                        instance_size as u32,
                        Some(#InstanceName::init),
                        gobject_ffi::GTypeFlags::empty()
                    );

                    // FIXME: add interfaces
                });

                TYPE
            }
        }
    }

    fn pub_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let pub_new_method = self.pub_new_method();

        quote! {
            impl #InstanceName {
                #pub_new_method
            }
        }
    }

    fn pub_new_method(&self) -> Tokens {
        let InstanceName = self.class.name;
        let imp_new_fn_name = self.imp_new_fn_name();

        // FIXME: we should take construct-only arguments and other convenient args to new()
        quote! {
            pub fn new() -> #InstanceName {
                unsafe { from_glib_full(imp::#imp_new_fn_name(/* FIXME: args */)) }
            }
        }
    }

    fn imp_new_fn_name(&self) -> Identifier {
        self.exported_fn_name("new")
    }

    fn imp_private_struct(&self) -> Tokens {
        let PrivateName = self.private_struct.name;
        let private_struct_fields = &self.private_struct.fields;
        let private_init_fn_body = &self.private_init_fn_body();

        quote! {
            struct #PrivateName {
                #(#private_struct_fields),*
            }

            impl #PrivateName {
                pub fn new() -> Self #private_init_fn_body
            }
        }
    }

    fn private_init_fn_body(&self) -> Tokens {
        // If the user had a "private_init()" method, we want to use it as an initializer
        // for the private struct.
        //
        // Otherwise, just initialize all of the struct's fields to Default::default().

        let private_init_member =
            self.class.members
            .iter()
            .filter_map(|m| match *m {
                Member::PrivateInit(ref f) => Some(f),
                _ => None,
            })
            .next();

        if let Some(i) = private_init_member {
            quote! { #i }
        } else {
            let PrivateName = self.private_struct.name;
            let private_struct_field_names =
                self.private_struct.fields
                                   .iter()
                                   .map(|f| f.name);
            quote! {
                {
                    #PrivateName {
                        #(#private_struct_field_names: Default::default()),*
                    }
                }
            }
        }
    }

    fn imp_class_private_struct(&self) -> Tokens {
        let PrivateClassName = &self.PrivateClassName;
        let ParentClassFfi = &self.ParentClassFfi;

        quote! {
            struct #PrivateClassName {
                parent_class: *const #ParentClassFfi,
                properties:   *const Vec<*const gobject_ffi::GParamSpec>,
                signals:      *const Vec<u32>
            }

            static mut PRIV: #PrivateClassName = #PrivateClassName {
                parent_class: ptr::null(),
                properties:   ptr::null(),
                signals:      ptr::null(),
            };
        }
    }

    fn imp_instance(&self) -> Tokens {
        let InstanceName = self.class.name;

        let all = vec![
            self.instance_get_class_fn(),
            self.instance_get_priv_fn(),
            self.instance_init_fn(),
            self.instance_finalize_fn(),
            // self.instance_set_property_fn(),
            // self.instance_get_property_fn(),
            // self.instance_method_trampolines()
            // self.instance_signal_trampolines()
            // self.instance_method_impls()
            // self.instance_default_signal_handlers()
        ];

        quote! {
            impl #InstanceName {
                #(#all)*
            }
        }
    }

    fn instance_get_class_fn(&self) -> Tokens {
        let GClassName = &self.GClassName;

        quote! {
            fn get_class(&self) -> &#GClassName {
                unsafe {
                    let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                    &*(klass as *const #GClassName)
                }
            }
        }
    }

    fn instance_get_priv_fn(&self) -> Tokens {
        let PrivateName = self.private_struct.name;
        let get_type_fn_name = self.get_type_fn_name();

        quote! {
            fn get_priv(&self) -> &#PrivateName {
                unsafe {
                    let private = gobject_ffi::g_type_instance_get_private(
                        self as *const _ as *mut gobject_ffi::GTypeInstance,
                        #get_type_fn_name(),
                    ) as *const Option<#PrivateName>;

                    (&*private).as_ref().unwrap()
                }
            }
        }
    }

    fn instance_init_fn(&self) -> Tokens {
        let callback_guard = self.callback_guard();
        let get_type_fn_name = self.get_type_fn_name();
        let PrivateName = self.private_struct.name;

        quote! {
            // Instance struct and private data initialization, called from GObject
            unsafe extern "C" fn init(obj: *mut gobject_ffi::GTypeInstance, _klass: glib_ffi::gpointer) {
                #callback_guard

                let private = gobject_ffi::g_type_instance_get_private(
                    obj as *mut gobject_ffi::GTypeInstance,
                    #get_type_fn_name(),
                ) as *mut Option<#PrivateName>;

                // Here we initialize the private data.  GObject gives it to us all zero-initialized
                // but we don't really want to have any Drop impls run here so just overwrite the
                // data.
                ptr::write(private, Some(#PrivateName::new()));
            }
        }
    }

    fn instance_finalize_fn(&self) -> Tokens {
        let callback_guard = self.callback_guard();
        let get_type_fn_name = self.get_type_fn_name();
        let PrivateName = self.private_struct.name;

        quote! {
            unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
                #callback_guard

                let private = gobject_ffi::g_type_instance_get_private(
                    obj as *mut gobject_ffi::GTypeInstance,
                    #get_type_fn_name(),
                ) as *mut Option<#PrivateName>;

                // Drop contents of private data by replacing its
                // Option container with None
                let _ = (*private).take();

                (*PRIV.parent_class).finalize.map(|f| f(obj));
            }
        }
    }

    fn imp_class(&self) -> Tokens {
        let GClassName = &self.GClassName;
        let class_init_fn = self.class_init_fn();

        quote! {
            impl #GClassName {
                #class_init_fn
                // FIXME
            }
        }
    }

    fn class_init_fn(&self) -> Tokens {
        let callback_guard = self.callback_guard();
        let InstanceName = self.class.name;
        let GClassName = &self.GClassName;
        let ParentClassFfi = &self.ParentClassFfi;
        let PrivateName = self.private_struct.name;
        let slot_assignments = self.slot_assignments();

        quote! {
            unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
                #callback_guard

                // This is an Option<_> so that we can replace its value with None on finalize() to
                // release all memory it holds
                gobject_ffi::g_type_class_add_private(klass, mem::size_of::<Option<#PrivateName>>());

                // GObjectClass methods; properties
                {
                    let gobject_class = &mut *(klass as *mut gobject_ffi::GObjectClass);
                    gobject_class.finalize = Some(#InstanceName::finalize);
                    // FIXME: gobject_class.set_property = Some(#InstanceName::set_property);
                    // FIXME: gobject_class.get_property = Some(#InstanceName::get_property);

                    // FIXME
                    // let mut properties = Vec::new();
                    //
                    // create each property

                }

                // Slots
                {
                    let klass = &mut *(klass as *mut #GClassName);
                    #(#slot_assignments)*
                }

                // Signals
                {
                    // FIXME
                }

                PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as *const #ParentClassFfi;
            }
        }
    }

    fn imp_extern_funcs(&self) -> Tokens {
        let imp_new_fn = self.imp_new_fn();

        quote! {
            #imp_new_fn
        }
    }

    fn imp_new_fn(&self) -> Tokens {
        let imp_new_fn_name = self.imp_new_fn_name();
        let InstanceName = self.class.name;
        let callback_guard = self.callback_guard();
        let get_type_fn_name = self.get_type_fn_name();

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #imp_new_fn_name(/* FIXME: args */) -> *mut #InstanceName {
                #callback_guard

                let this = gobject_ffi::g_object_newv(
                    #get_type_fn_name(),
                    0,
                    ptr::null_mut()
                );

                this as *mut #InstanceName
            }
        }
    }

    fn instance_ext(&self) -> Tokens {
        let InstanceExt = self.InstanceExt;
        let method_trait_fns = &self.method_trait_fns();

        quote! {
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
        }
    }

    fn instance_ext_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let InstanceExt = self.InstanceExt;
        let method_redirects = self.method_redirects();
        quote! {
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
        }
    }

    fn signal_trampolines(&self) -> Tokens {
        let InstanceName = self.class.name;
        let InstanceExt = self.InstanceExt;

        quote! {
            // FIXME: signal handler trampolines like in glib-rs
            //
            // unsafe extern "C" fn signalname_trampoline<P>(this: *mut ffi::InstanceName, argname: type, argname: type, f: glib_ffi:gpointer) -> type
            // where P: IsA<InstanceName> {
            //     callback_guard!();
            //     let f: &&(Fn(&P, type, type) -> type + 'static) = transmute(f);
            //
            //     // with return value:
            //     f(&InstanceName::from_glib_none(this).downcast_unchecked(), &from_glib_none(argname), &from_glib_none(argname))).to_glib()
            //
            //     // without return value:
            //     f(&InstanceName::from_glib_none(this).downcast_unchecked(), &from_glib_none(argname), &from_glib_none(argname)))
            //
            //     // those are by-reference arguments.  For by-value arguments,
            //     from_glib(argname)
            // }
        }
    }
/*
    fn impls(&self) -> Tokens {
        let InstanceName = self.class.name;
        let FieldsName = self.FieldsName;
        let GClassName = self.GClassName;
        let GObject = &self.GObject;
        let ParentClassFfi = &self.ParentClassFfi;

        let get_type_fn = self.get_type_fn();

        quote! {
            unsafe impl ::gnome_class_shims::GInstance for #InstanceName {
                type Class = #GClassName;
                type Fields = #FieldsName;

                #get_type_fn

                unsafe fn to_gobject_ptr(this: *const Self)
                                         -> *mut #GObject {
                    (*this).parent
                }

                unsafe fn from_gobject_ptr(g: *mut #GObject) -> Self {
                    #InstanceName { parent: g }
                }
            }

            unsafe impl ::gnome_class_shims::GClass for #GClassName {
                type Instance = #InstanceName;
            }

            unsafe impl ::gnome_class_shims::GSubclass for #GClassName {
                type ParentClass = #ParentClassFfi;
            }

            unsafe impl ::gnome_class_shims::GFields for #FieldsName {
                type Instance = #InstanceName;
            }

            impl Clone for #InstanceName {
                fn clone(&self) -> Self {
                    use gnome_class_shims::GInstance;
                    GInstance::clone_ref(self)
                }
            }

            impl Drop for #InstanceName {
                fn drop(&mut self) {
                    // We assert that we are calling `drop_ref` from
                    // within `drop`:
                    use gnome_class_shims::GInstance;
                    unsafe {
                        GInstance::drop_ref(self)
                    }
                }
            }
        }
    }
*/

    pub fn methods(&self) -> impl Iterator<Item = &'ast Method> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(ref m) => Some(m),
                _ => None,
            })
    }

    pub fn signals(&self) -> impl Iterator<Item = &'ast Signal> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Signal(ref s) => Some(s),
                _ => None,
            })
    }

    pub fn members_with_slots(&self) -> impl Iterator<Item = &'ast Member> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(_) => Some(member),
                Member::Signal(_) => Some(member),
                _ => None,
            })
    }

    fn slot_assignments(&self) -> Vec<Tokens> {
        let InstanceName = self.class.name;

        self.members_with_slots()
            .map(|member| {
                let slot_name = match *member {
                    Member::Method(ref method) => method.name,
                    Member::Signal(ref signal) => signal.name,
                    _ => unreachable!()
                };

                let slot_trampoline = Identifier {
                    str: intern(&format!("{}_trampoline", slot_name.str))
                };

                quote! {
                    klass.#slot_name = Some(#InstanceName::#slot_trampoline);
                }
            })
            .collect()
    }

    /// From a signal called `foo`, generate `foo_signal_id`.  This is used to
    /// store the signal ids from g_signal_newv() in the Class structure.
    pub fn signal_id_names(&self) -> Vec<Identifier> {
        self.signals()
            .map(|signal| signal_id_name (signal))
            .collect()
    }

    pub fn method_names(&self) -> Vec<Identifier> {
        self.methods()
            .map(|method| method.name)
            .collect()
    }

    fn method_assignments(&self) -> Vec<Tokens> {
        let InstanceName = self.class.name;
        let MethodsFrom = &self.MethodsFrom;
        self.method_names()
            .iter()
            .map(|method_name| {
                quote! { klass.#method_name = Some(<#InstanceName as #MethodsFrom::Trait>::#method_name); }
            })
            .collect()
    }

    fn signal_declarations(&self) -> Vec<Tokens> {
        self.signals()
            .map(|signal| {
                // FIXME: we are not specifying the proper signature (return, args) for the signal
                // handler.  We need a way to translate Rust type names into G_TYPE_* ids.
                //
                // FIXME: we are not passing signal flags
                //
                // FIXME: We are not passing a class_closure, marshaller, etc.

                let signal_id_name = signal_id_name(&signal);
                let signal_name = ByteString(signal.name);
                quote! {
                    klass.#signal_id_name =
                        gobject_sys::g_signal_newv (#signal_name as *const u8 as *const i8,
                                                    (*g_object_class).g_type_class.g_type,
                                                    gobject_sys::G_SIGNAL_RUN_FIRST, // flags
                                                    ptr::null_mut(),                 // class_closure,
                                                    None,                            // accumulator
                                                    ptr::null_mut(),                 // accu_data
                                                    None,                            // c_marshaller,
                                                    gobject_sys::G_TYPE_NONE,        // return_type
                                                    0,                               // n_params,
                                                    ptr::null_mut()                  // param_types
                        );
                }
            })
            .collect()
    }

    pub fn method_fn_tys(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let method_fn_ty = SlotTy {
                    class_name: self.class.name,
                    sig: &method.fn_def.sig
                };
                quote! { #method_fn_ty }
            })
            .collect()
    }

    fn methods_declared_in_instance(&self) -> Tokens {
        let InstanceName = self.class.name;
        let method_trait_fns = &self.method_trait_fns();
        let method_impl_fns = &self.method_impl_fns();
        let MethodsFrom = &self.MethodsFrom;

        quote! {
            #[allow(non_snake_case)]
            mod #MethodsFrom {
                use super::*;
                pub trait Trait {
                    #(extern #method_trait_fns)*
                }
            }

            impl #MethodsFrom::Trait for #InstanceName {
                #(extern #method_impl_fns)*
            }
        }
    }

    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32);
    /// ```
    pub fn method_trait_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty;
                }
            })
            .collect()
    }

    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32) { ... }
    /// ```
    pub fn method_impl_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let return_ty = method.fn_def.sig.return_ty();
                let code = &method.fn_def.code;
                quote! {
                    fn #name(&self, #arg_decls) #return_ty #code
                }
            })
            .collect()
    }

    fn method_redirects(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let ffi_name = self.method_ffi_name(method);
                let arg_decls = method.fn_def.sig.arg_decls();
                let arg_names = method.fn_def.sig.arg_names();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty {
                        unsafe { imp::#ffi_name(self.to_glib_none().0, #arg_names) }
                    }
                }
            })
            .collect()
    }

    fn method_ffi_name(&self, method: &Method) -> Identifier {
        self.exported_fn_name(&method.name.str.to_string())
    }

    fn lower_case_class_name(&self) -> String {
        lalrpop_intern::read(|interner| {
            let name_str = interner.data(self.class.name.str);
            let mut name_chars = name_str.chars();
            let first_char: char = name_chars.next().unwrap();
            first_char.to_lowercase().chain(name_chars).collect()
        })
    }

    fn c_symbols(&self) -> Tokens {
        let InstanceName = self.class.name;
        let FieldsName = self.FieldsName;
        let instanceName = self.lower_case_class_name();

        let method_tokens: Vec<_> =
            self.methods()
                .map(|method| {
                    let arg_decls = method.fn_def.sig.arg_decls();
                    let arg_names = method.fn_def.sig.arg_names();
                    let return_ty = method.fn_def.sig.return_ty();
                    let name = method.name;
                    let c_name = Ident::new(format!("{}_{}",
                                                    instanceName,
                                                    method.name.str));
                    quote! {
                        #[no_mangle]
                        pub extern fn #c_name(__this: &#FieldsName, #arg_decls) #return_ty {
                            use gnome_class_shims::GFields;
                            let __this = <#FieldsName as GFields>::as_instance(&__this);
                            #InstanceName::#name(__this, #arg_names)
                        }
                    }
                })
                .collect();

        let get_type_name = Ident::new(format!("{}_get_type",
                                               instanceName));
        quote! {
            #[no_mangle]
            pub extern fn #get_type_name() -> ::gnome_class_shims::glib_sys::GType
            {
                use gnome_class_shims::GInstance;
                <#InstanceName as GInstance>::get_type()
            }

            #(#method_tokens)*
        }
    }

    fn get_type_fn(&self) -> Tokens {
        let callback_guard = self.callback_guard();
        let InstanceName = self.class.name;
        let FieldsName = self.GClassName;
        let GClassName = self.GClassName;
        let ParentInstance = &self.ParentInstance;
        let PrivateName = self.private_struct.name;

        // The function which initializes an instance of our class.
        // It simply sets up the private fields.
        let instance_init = quote! {
            extern fn instance_init(this: *mut GTypeInstance,
                                    _klass: gpointer)
            {
                unsafe {
                    let private = gobject_sys::g_type_instance_get_private(this, #InstanceName::get_type());
                    let private = private as *mut #PrivateName;
                    ptr::write(private, #PrivateName::new());
                }
            }
        };

        // The finalizer. It drops the private fields and then invokes
        // the parent class finalizer (which it loads from the parent
        // class struct).
        let finalize = quote! {
            extern fn finalize(this: *mut GObject) {
                use gnome_class_shims::GInstance;

                unsafe {
                    let this = <#InstanceName as GInstance>::borrow_gobject_ptr(&this);
                    ptr::read((*this).private());

                    let object_class = shims::get_class(&*this);
                    let parent_class = shims::get_parent_class(object_class);
                    if let Some(f) = parent_class.finalize {
                        f(#InstanceName::to_gobject_ptr(this))
                    }
                }
            }
        };

        // Class initializer. Sets up the finalizer, private field
        // size, initializes the fields for each of our methods,
        // and registers the signals.
        let method_assignments = self.method_assignments();
        let signal_declarations = self.signal_declarations();

        let class_init = quote! {
            extern fn class_init(klass: gpointer,
                                 _klass_data: gpointer)
            {
                unsafe {
                    let g_object_class = klass as *mut GObjectClass;
                    (*g_object_class).finalize = Some(finalize);

                    gobject_sys::g_type_class_add_private(
                        klass,
                        mem::size_of::<#PrivateName>());

                    let klass = klass as *mut #GClassName;
                    let klass: &mut #GClassName = &mut *klass;
                    #(#method_assignments)*

                    #(#signal_declarations)*
                }
            }
        };

        // Registration function. Creates the GType. Intended to be run
        // at most once, returns the `GType` we created.
        let byte_string = ByteString(self.class.name);
        let register = quote! {
            fn register() -> GType {
                use std::u16;
                unsafe {
                    let class_size = mem::size_of::<#GClassName>();
                    assert!(class_size <= u16::MAX as usize);

                    let instance_size = mem::size_of::<#FieldsName>();
                    assert!(instance_size <= u16::MAX as usize);

                    gobject_sys::g_type_register_static_simple(
                        #ParentInstance::get_type(),
                        #byte_string as *const u8 as *const i8,
                        class_size as u32,
                        Some(class_init),
                        instance_size as u32,
                        Some(instance_init),
                        GTypeFlags::empty())
                }
            }
        };

        quote! {
            fn get_type() -> ::gnome_class_shims::glib_sys::GType {
                #callback_guard

                use gnome_class_shims as shims;
                use gnome_class_shims::gobject_sys::{self,
                                                     GObject,
                                                     GObjectClass,
                                                     GTypeInstance,
                                                     GTypeFlags};
                use gnome_class_shims::glib_sys::{GType, gpointer};
                use std::{mem, ptr};
                use std::sync::{Once, ONCE_INIT};

                // All these helper functions are intentionally
                // hidden inside of `get_type` so as not to
                // pollute the user's namespace.
                #instance_init
                #finalize
                #class_init
                #register

                unsafe {
                    static mut GTYPE: GType = gobject_sys::G_TYPE_INVALID;
                    static ONCE: Once = ONCE_INIT;

                    ONCE.call_once(|| {
                        GTYPE = register();
                    });

                    GTYPE
                }
            }
        }
    }
}

impl ToTokens for VarTy {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &VarTy { name, ref ty } = self;
        quote_in!(tokens, #name: #ty)
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Type::Path(ref path) => {
                path.ty().to_tokens(tokens)
            }
            Type::Array(ref ty) => {
                quote_in!(tokens, [ #ty ]);
            }
            Type::Sum(ref tys) => {
                quote_in!(tokens, #(#tys)+*);
            }
        }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut Tokens) {
        lalrpop_intern::read(|interner| {
            Ident::new(interner.data(self.str)).to_tokens(tokens);
        })
    }
}

struct ByteString(Identifier);

impl ToTokens for ByteString {
    fn to_tokens(&self, tokens: &mut Tokens) {
        lalrpop_intern::read(|interner| {
            // Because we are converting a legal identifier, we don't
            // have to worry about it having escape characters in it
            // or anything else:
            let mut s = String::new();
            s.push_str("b\"");
            s.push_str(interner.data(self.0.str));
            s.push_str("\\0\"");
            tokens.append(&s);
        })
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.tokens.to_tokens(tokens)
    }
}

struct SlotTy<'ast> {
    class_name: Identifier,
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for SlotTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let class_name = self.class_name;
        let arg_decls = self.sig.arg_decls();
        let return_ty = self.sig.return_ty();

        quote_in! {
            tokens,
            (this: *mut #class_name, #arg_decls) #return_ty
        }
    }
}

/// Helper methods for printing out various bits of
/// a method signature. For each of the comments below,
/// assume an example method `fn get(&self, x: u32, y: u32) -> u32`.
impl FnSig {
    /// Generates `x: u32, y: u32`
    fn arg_decls<'ast>(&'ast self) -> ArgDecls<'ast> {
        ArgDecls { sig: self }
    }

    /// Generates `x, y`
    fn arg_names<'ast>(&'ast self) -> ArgNames<'ast> {
        ArgNames { sig: self }
    }

    /// Generates `-> u32` (or `` if unit)
    fn return_ty<'ast>(&'ast self) -> ReturnTy<'ast> {
        ReturnTy { sig: self }
    }
}

struct ArgDecls<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgDecls<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = &self.sig.args;
        quote_in!(tokens, #(#args),*);
    }
}

struct ArgNames<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgNames<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = self.sig.args.iter().map(|arg| arg.name);
        quote_in!(tokens, #(#args),*);
    }
}

struct ReturnTy<'ast> {
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for ReturnTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if let Some(ref return_ty) = self.sig.return_ty {
            quote_in!(tokens, -> #return_ty)
        }
    }
}

impl Path {
    fn ty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: false, path: self }
    }

    fn exprty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: true, path: self }
    }
}

struct SepPath<'a> {
    cc: bool,
    path: &'a Path,
}

impl<'a> ToTokens for SepPath<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self.path {
            Path::FromRoot => tokens.append("::"),
            Path::FromSelf => tokens.append("self"),
            Path::FromSuper => tokens.append("super"),
            Path::FromTraitItem(ref i) => i.to_tokens(tokens),
            Path::From(ref i) => {
                let i = SepPathId { cc: self.cc, path_id: i};
                i.to_tokens(tokens)
            }
            Path::Extend(ref b, ref i) => {
                let b = SepPath { cc: self.cc, path: b };
                let i = SepPathId { cc: self.cc, path_id: i };
                quote_in!(tokens, #b :: #i)
            }
        }
    }
}

struct SepPathId<'a> {
    cc: bool,
    path_id: &'a PathId,
}

impl<'a> ToTokens for SepPathId<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.path_id.name.to_tokens(tokens);
        let tys = &self.path_id.tys;
        if !tys.is_empty() {
            if self.cc {
                tokens.append("::");
            }
            quote_in!(tokens, <#(#tys),*>);
        }
    }
}

impl ToTokens for TraitItemId {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &TraitItemId { ref self_ty, ref trait_ref, item } = self;
        let trait_ref = trait_ref.ty();
        quote_in!(tokens, < #self_ty as #trait_ref > :: #item);
    }
}
