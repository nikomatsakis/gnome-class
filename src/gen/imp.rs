use super::*;
use cstringident::*;

impl<'ast> ClassContext<'ast> {
    fn imp_module(&self) -> Tokens {
        let all = vec![
            self.imp_instance_struct(),
            self.imp_class_struct(),
            self.imp_properties_enum(),
            self.imp_signals_enum(),
            self.imp_private_struct(),
            self.imp_class_private_struct(),
            self.imp_slot_impls(),
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
        let InstanceName = self.InstanceName;
        let ParentInstanceFfi = &self.ParentInstanceFfi;

        quote! {
            #[repr(C)]
            pub struct #InstanceName {
                pub parent: #ParentInstanceFfi,
            }
        }
    }

    fn imp_class_struct(&self) -> Tokens {
        let ClassName = self.ClassName;
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
                                                       class_name: self.InstanceName,
                                                       sig: &method.fn_def.sig
                                                   }),

                    Member::Signal(ref signal) => (signal.name,
                                                   SlotTy {
                                                       class_name: self.InstanceName,
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
            // FIXME: self.private_struct.fields is no longer Vec<VarTy>; it is now syn::VariantData.
            let private_struct_field_names =
                self.private_struct.fields
                                   .iter()
                                   .map(|f| f.ident.as_ref().unwrap());
            quote! {
                {
                    #PrivateName {
                        #(#private_struct_field_names: Default::default()),*
                    }
                }
            }
        }
    }

    fn imp_private_struct(&self) -> Tokens {
        // FIXME: self.private_struct now only has a .derive_input
        let PrivateName = &self.private_struct.derive_input.ident;
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
                // we use this instead of "ptr::null()" because using
                // function calls to set constants is feature-gated.
                parent_class: 0 as *const _,
                properties:   0 as *const _,
                signals:      0 as *const _,
            };
        }
    }

    fn imp_get_priv_fn(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let PrivateName = self.private_struct.name;
        let get_type_fn_name = self.instance_get_type_fn_name();

        quote! {
            fn get_priv(&self) -> &#PrivateName {
                unsafe {
                    let private = gobject_ffi::g_type_instance_get_private(
                        <Self as ToGlibPtr<*mut #InstanceName>>::to_glib_none(self).0 as *mut gobject_ffi::GTypeInstance,
                        #get_type_fn_name(),
                    ) as *const Option<#PrivateName>;

                    (&*private).as_ref().unwrap()
                }
            }
        }
    }

    fn imp_slot_impls(&self) -> Tokens {
        // We are inside the "mod imp".  We will create function
        // implementations for the default handlers for methods and
        // signals as "impl super::Foo { ... }", so that the &self in
        // those functions will refer to the Rust wrapper object that
        // corresponds to the GObject-ABI structs within "mod imp".

        let InstanceName = self.InstanceName;
        let get_priv = self.imp_get_priv_fn();
        let impls = self.imp_slot_default_handlers();

        quote! {
            impl super::#InstanceName {
                #get_priv

                #(#impls)*
            }
        }
    }

    fn imp_slot_default_handlers(&self) -> Vec<Tokens> {
        self.members_with_slots()
            .map(|member| {
                let (slot_name, slot_impl_ty, code) = match *member {
                    Member::Method(ref method) => (&method.name,
                                                   SlotImplTy {
                                                       sig:  &method.fn_def.sig
                                                   },
                                                   &method.fn_def.code),

                    Member::Signal(ref signal) => (&signal.name,
                                                   SlotImplTy {
                                                       sig:  &signal.sig
                                                   },
                                                   signal.code.as_ref().unwrap()), // FIXME: signals with no default handler?
                    _ => unreachable!()
                };

                let slot_impl_name = Self::slot_impl_name(slot_name);

                quote! {
                    fn #slot_impl_name#slot_impl_ty #code
                }
            })
            .collect()
    }

    fn instance_init_fn(&self) -> Tokens {
        let callback_guard = self.glib_callback_guard();
        let get_type_fn_name = self.instance_get_type_fn_name();
        let PrivateName = self.private_struct.name;

        quote! {
            // Instance struct and private data initialization, called from GObject
            unsafe extern "C" fn init(obj: *mut gobject_ffi::GTypeInstance, _klass: glib_ffi::gpointer) {
                #callback_guard

                let private = gobject_ffi::g_type_instance_get_private(
                    obj,
                    #get_type_fn_name()
                ) as *mut Option<#PrivateName>;

                // Here we initialize the private data.  GObject gives it to us all zero-initialized
                // but we don't really want to have any Drop impls run here so just overwrite the
                // data.
                ptr::write(private, Some(#PrivateName::new()));
            }
        }
    }

    fn instance_finalize_fn(&self) -> Tokens {
        let callback_guard = self.glib_callback_guard();
        let get_type_fn_name = self.instance_get_type_fn_name();
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

    fn instance_method_trampolines(&self) -> Tokens {
        let callback_guard = self.glib_callback_guard();
        let InstanceName = self.InstanceName;

        let impls: Vec<Tokens> = self.methods()
            .map(|method| {
                let trampoline_name = Self::slot_trampoline_name(&method.name);
                let method_impl_name = Self::slot_impl_name(&method.name);

                let slot_ty = SlotTy {
                    class_name: self.InstanceName,
                    sig: &method.fn_def.sig
                };

                let arg_names = method.fn_def.sig.arg_names();

                quote! {
                    unsafe extern "C" fn #trampoline_name#slot_ty {
                        #callback_guard

                        let instance: &super::#InstanceName = &from_glib_borrow(this);

                        // FIXME: do we need to from_glib_*() each argument?
                        // FIXME: do we need to .to_glib() the return value?
                        instance.#method_impl_name(#arg_names)
                    }
                }
            })
            .collect();

        quote! {
            #(#impls)*
        }
    }

    fn imp_instance(&self) -> Tokens {
        let InstanceName = self.InstanceName;

        let all = vec![
            self.instance_get_class_fn(),
            self.instance_init_fn(),
            self.instance_finalize_fn(),
            // self.instance_set_property_fn(),
            // self.instance_get_property_fn(),
            self.instance_method_trampolines()
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

    fn imp_class(&self) -> Tokens {
        let ClassName = &self.ClassName;
        let class_init_fn = self.class_init_fn();

        quote! {
            impl #ClassName {
                #class_init_fn
                // FIXME
            }
        }
    }

    fn class_init_fn(&self) -> Tokens {
        let callback_guard = self.glib_callback_guard();
        let InstanceName = self.InstanceName;
        let ClassName = &self.ClassName;
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
                    let klass = &mut *(klass as *mut #ClassName);
                    #(#slot_assignments)*
                }

                // Signals
                {
                    // FIXME self.signal_declarations();
                }

                PRIV.parent_class = gobject_ffi::g_type_class_peek_parent(klass) as *const #ParentClassFfi;
            }
        }
    }

    fn imp_new_fn(&self) -> Tokens {
        let imp_new_fn_name = self.imp_new_fn_name();
        let InstanceName = self.InstanceName;
        let callback_guard = self.glib_callback_guard();
        let get_type_fn_name = self.instance_get_type_fn_name();

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

    fn imp_extern_methods(&self) -> Vec<Tokens> {
        let InstanceName = self.InstanceName;
        let callback_guard = self.glib_callback_guard();

        self.methods()
            .map(|method| {
                let arg_decls   = method.fn_def.sig.arg_decls();
                let arg_names   = method.fn_def.sig.arg_names();
                let return_ty   = method.fn_def.sig.return_ty();
                let method_name = method.name;
                let ffi_name    = self.method_ffi_name(method);

                quote! {
                    #[no_mangle]
                    pub unsafe extern "C" fn #ffi_name(this: *mut #InstanceName, #arg_decls) #return_ty {
                        #callback_guard

                        let klass = (*this).get_class();
                        // We unwrap() because klass.method_name is always set to a method_trampoline
                        (klass.#method_name.as_ref().unwrap())(this, #arg_names)
                    }
                }
            })
            .collect()
    }

    fn imp_extern_funcs(&self) -> Tokens {
        let imp_new_fn = self.imp_new_fn();
        let imp_extern_methods = self.imp_extern_methods();

        quote! {
            #imp_new_fn
            #(#imp_extern_methods)*
        }
    }

    fn instance_get_class_fn(&self) -> Tokens {
        let ClassName = &self.ClassName;

        quote! {
            fn get_class(&self) -> &#ClassName {
                unsafe {
                    let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
                    &*(klass as *const #ClassName)
                }
            }
        }
    }

    fn imp_get_type_fn(&self) -> Tokens {
        let callback_guard = self.glib_callback_guard();
        let get_type_fn_name = self.instance_get_type_fn_name();
        let ClassName = self.ClassName;
        let InstanceName = self.InstanceName;
        let ParentInstance = &self.ParentInstance;
        let instance_name_string = CStringIdent(InstanceName);

        quote! {
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

                    let instance_size = mem::size_of::<#InstanceName>();
                    assert!(instance_size <= u16::MAX as usize);

                    TYPE = gobject_ffi::g_type_register_static_simple(
                        <#ParentInstance as glib::StaticType>::static_type().to_glib(),
                        #instance_name_string as *const u8 as *const i8,
                        class_size as u32,
                        Some(#ClassName::init),
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

    fn slot_trampoline_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_trampoline", slot_name.as_ref()))
    }

    fn slot_impl_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_impl", slot_name.as_ref()))
    }

    fn slot_assignments(&self) -> Vec<Tokens> {
        let InstanceName = self.InstanceName;

        self.members_with_slots()
            .map(|member| {
                let slot_name = match *member {
                    Member::Method(ref method) => method.name,
                    Member::Signal(ref signal) => signal.name,
                    _ => unreachable!()
                };

                let trampoline_name = Self::slot_trampoline_name(&slot_name);

                quote! {
                    klass.#slot_name = Some(#InstanceName::#trampoline_name);
                }
            })
            .collect()
    }

    fn imp_new_fn_name(&self) -> Ident {
        self.exported_fn_name("new")
    }
}
