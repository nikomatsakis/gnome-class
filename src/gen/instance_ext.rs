use super::*;

impl<'ast> ClassContext<'ast> {
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
        let InstanceName = self.InstanceName;
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

    fn method_ffi_name(&self, method: &Method) -> Ident {
        self.exported_fn_name(method.name.as_ref())
    }

    pub fn methods(&self) -> impl Iterator<Item = &'ast Method> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(ref m) => Some(m),
                _ => None,
            })
    }
}
