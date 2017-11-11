use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn slots(&self) -> Vec<Tokens> {
        // ABI: we are generating the imp::FooClass with the parent_class, and the slots to signals/methods.
        // This defines the C ABI for the class structure.
        //
        // FIXME: we should check that the extern "C" signatures only have types representable by C.

        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(_) => None,

                    Slot::VirtualMethod(VirtualMethod { name, inputs, output, .. }) => {
                        let inputs = &inputs[1..];
                        let InstanceName = &self.InstanceName;
                        Some(quote! {
                            pub #name: Option<unsafe extern "C" fn(
                                this: *mut #InstanceName,
                                #(#inputs),*
                            ) #output>,

                        })
                    }

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn imp_slot_default_handlers(&self) -> Vec<Tokens> {
        self.class
            .slots
            .iter()
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { public: false, name, inputs, output, body }) => {
                        quote! {
                            fn #name(#(#inputs),*) #output #body
                        }
                    },
                    Slot::Method(Method { name, inputs, output, body, .. }) |
                    Slot::VirtualMethod(VirtualMethod { name, inputs, output, body: Some(body), .. }) => {
                        let name = Self::slot_impl_name(&name);
                        quote! {
                            fn #name(#(#inputs),*) #output #body
                        }
                    },

                    Slot::VirtualMethod(VirtualMethod { name, inputs, output, body: None, .. }) => {
                        let name = Self::slot_impl_name(&name);
                        quote! {
                            fn #name(#(#inputs),*) #output {
                                panic!("Called abstract method {} with no implementation", stringify!(#name));
                            }
                        }
                    },

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn instance_method_trampolines(&self) -> Vec<Tokens> {
        let callback_guard = glib_callback_guard();
        let InstanceName = self.InstanceName;
        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(_) => None,

                    Slot::VirtualMethod(VirtualMethod { name, inputs, output, .. }) => {
                        let trampoline_name = Self::slot_trampoline_name(&name);
                        let method_impl_name = Self::slot_impl_name(&name);
                        let inputs = &inputs[1..];
                        let arg_names = ArgNames(inputs);
                        Some (quote! {
                            unsafe extern "C" fn #trampoline_name(this: *mut #InstanceName,
                                                                  #(#inputs),*)
                                #output
                            {
                                #callback_guard

                                let instance: &super::#InstanceName = &from_glib_borrow(this);

                                // FIXME: do we need to from_glib_*() each argument?
                                // FIXME: do we need to .to_glib() the return value?
                                instance.#method_impl_name(#arg_names)
                            }
                        })
                    },

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn instance_signal_trampolines(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn instance_method_impls(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn instance_default_signal_handlers(&self) -> Vec<Tokens> {
        // FIXME
        Vec::new()
    }

    pub fn imp_extern_methods(&self) -> Vec<Tokens> {
        let InstanceName = self.InstanceName;
        let callback_guard = glib_callback_guard();
        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(Method { public: false, .. }) => None, // these don't get exposed in the C API

                    Slot::Method(Method { public: true, name, inputs, output, .. }) => {
                        let ffi_name = self.method_ffi_name(name.sym.as_str());
                        let method_impl_name = Self::slot_impl_name(&name);
                        let inputs = &inputs[1..];
                        let arg_names = ArgNames(inputs);
                        Some(quote! {
                            #[no_mangle]
                            pub unsafe extern "C" fn #ffi_name(this: *mut #InstanceName, #(#inputs),*) #output {
                                #callback_guard

                                let instance: &super::#InstanceName = &from_glib_borrow(this);

                                // FIXME: do we need to from_glib_*() each argument?
                                // FIXME: do we need to .to_glib() the return value?
                                instance.#method_impl_name(#arg_names)
                            }
                        })
                    },

                    Slot::VirtualMethod(VirtualMethod { name, inputs, output, .. }) => {
                        let ffi_name = self.method_ffi_name(name.sym.as_str());
                        let inputs = &inputs[1..];
                        let arg_names = ArgNames(inputs);
                        Some(quote! {
                            #[no_mangle]
                            pub unsafe extern "C" fn #ffi_name(this: *mut #InstanceName, #(#inputs),*) #output {
                                #callback_guard

                                let klass = (*this).get_class();
                                // We unwrap() because klass.method_name is always set to a method_trampoline
                                (klass.#name.as_ref().unwrap())(this, #arg_names)
                            }
                        })
                    },

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    fn slot_trampoline_name(slot_name: &Ident) -> Ident {
        Ident::from(format!("{}_trampoline", slot_name.as_ref()))
    }

    fn slot_impl_name(slot_name: &Ident) -> Ident {
        Ident::from(format!("{}_impl", slot_name.as_ref()))
    }

    pub fn slot_assignments(&self) -> Vec<Tokens> {
        let InstanceName = &self.InstanceName;
        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(_) => None,

                    Slot::VirtualMethod(VirtualMethod { name, .. }) => {
                        let trampoline_name = Self::slot_trampoline_name(&name);

                        Some(quote! {
                            klass.#name = Some(#InstanceName::#trampoline_name);
                        })
                    }

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn imp_new_fn_name(&self) -> Ident {
        self.exported_fn_name("new")
    }

    pub fn register_instance_private(&self) -> Tokens {
        match self.instance_private {
            Some(name) => {
                let PrivateName = name;

                quote! {
                    // This is an Option<_> so that we can replace its value with None on finalize() to
                    // release all memory it holds
                    gobject_ffi::g_type_class_add_private(klass, mem::size_of::<Option<#PrivateName>>());
                }
            },

            None => {
                quote! {}
            }
        }
    }

    pub fn get_priv_fn(&self) -> Tokens {
        match self.instance_private {
            Some(name) => {
                let PrivateName = name;
                let InstanceName = self.InstanceName;
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
            },

            None => quote! {}
        }
    }

    pub fn init_priv_with_default(&self) -> Tokens {
        match self.instance_private {
            Some(name) => {
                let PrivateName = name;
                let get_type_fn_name = self.instance_get_type_fn_name();

                quote! {
                    let private = gobject_ffi::g_type_instance_get_private(
                        obj,
                        #get_type_fn_name()
                    ) as *mut Option<#PrivateName>;

                    // Here we initialize the private data.  GObject gives it to us all zero-initialized
                    // but we don't really want to have any Drop impls run here so just overwrite the
                    // data.
                    ptr::write(private, Some(<#PrivateName as Default>::default()));
                }
            },

            None => quote! {}
        }
    }

    pub fn free_instance_private(&self) -> Tokens {
        match self.instance_private {
            Some(name) => {
                let PrivateName = name;
                let get_type_fn_name = self.instance_get_type_fn_name();

                quote! {
                    let private = gobject_ffi::g_type_instance_get_private(
                        obj as *mut gobject_ffi::GTypeInstance,
                        #get_type_fn_name(),
                    ) as *mut Option<#PrivateName>;

                    // Drop contents of private data by replacing its
                    // Option container with None
                    let _ = (*private).take();
                }
            },

            None => quote! {}
        }
    }
}
