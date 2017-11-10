use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn slots(&self) -> Vec<Tokens> {
        // ABI: we are generating the imp::FooClass with the parent_class, and the slots to signals/methods.
        // This defines the C ABI for the class structure.
        //
        // FIXME: we should check that the extern "C" signatures only have types representable by C.

        /*
        self.members_with_slots()
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
            .collect()
        */
        self.class
            .slots
            .iter()
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { name, inputs, output, .. }) => {
                        let inputs = &inputs[1..];
                        let InstanceName = &self.InstanceName;
                        quote! {
                            pub #name: Option<unsafe extern "C" fn(
                                this: *mut #InstanceName,
                                #(#inputs),*
                            ) #output>,
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
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
                    Slot::Method(Method { public, name, inputs, output, body }) => {
                        drop(public); // TODO: use this?
                        let name = Self::slot_impl_name(&name);
                        quote! {
                            fn #name(#(#inputs),*) #output #body
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
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
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { name, inputs, output, .. }) => {
                        let trampoline_name = Self::slot_trampoline_name(&name);
                        let method_impl_name = Self::slot_impl_name(&name);
                        let inputs = &inputs[1..];
                        let arg_names = ArgNames(inputs);
                        quote! {
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
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
        // self.methods()
        //     .map(|method| {
        //         let trampoline_name = Self::slot_trampoline_name(&method.name);
        //         let method_impl_name = Self::slot_impl_name(&method.name);
        //
        //         let slot_ty = SlotTy {
        //             class_name: self.InstanceName,
        //             sig: &method.fn_def.sig
        //         };
        //
        //         let arg_names = method.fn_def.sig.arg_names();

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
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { public, name, inputs, output, body: _ }) => {
                        let ffi_name = self.method_ffi_name(name.sym.as_str());
                        let inputs = &inputs[1..];
                        let arg_names = ArgNames(inputs);
                        drop(public); // TODO: use this?
                        quote! {
                            #[no_mangle]
                            pub unsafe extern "C" fn #ffi_name(this: *mut #InstanceName, #(#inputs),*) #output {
                                #callback_guard

                                let klass = (*this).get_class();
                                // We unwrap() because klass.method_name is always set to a method_trampoline
                                (klass.#name.as_ref().unwrap())(this, #arg_names)
                            }
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
        /*
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
         */
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
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { name, .. }) => {
                        let trampoline_name = Self::slot_trampoline_name(&name);

                        quote! {
                            klass.#name = Some(#InstanceName::#trampoline_name);
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn imp_new_fn_name(&self) -> Ident {
        self.exported_fn_name("new")
    }
}
