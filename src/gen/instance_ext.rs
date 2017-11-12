use super::*;

impl<'ast> ClassContext<'ast> {
    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32);
    /// ```
    pub fn method_trait_fns(&self) -> Vec<Tokens> {
        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(Method { public: false, .. }) => None,

                    Slot::Method(Method { public: true, ref sig, .. }) |
                    Slot::VirtualMethod(VirtualMethod { ref sig, .. }) => {
                        let name = sig.name;
                        let inputs = &sig.inputs;
                        let output = &sig.output;
                        Some(quote! {
                            fn #name(#(#inputs),*) -> #output;
                        })
                    }

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn method_redirects(&self) -> Vec<Tokens> {
        self.class
            .slots
            .iter()
            .filter_map(|slot| {
                match *slot {
                    Slot::Method(Method { public: false, .. }) => None,

                    Slot::Method(Method { public: true, ref sig, .. }) |
                    Slot::VirtualMethod(VirtualMethod { ref sig, .. }) => {
                        let name = sig.name;
                        let ffi_name = self.method_ffi_name(name.sym.as_str());
                        let arg_names = sig.input_args_to_glib_types();
                        let output_from = sig.ret_from_glib_fn();
                        let inputs = &sig.inputs;
                        let output = &sig.output;
                        Some(quote! {
                            fn #name(#(#inputs),*) -> #output {
                                #output_from(unsafe {
                                    imp::#ffi_name(self.to_glib_none().0,
                                                   #arg_names)
                                })
                            }
                        })
                    }

                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
    }

    pub fn method_ffi_name(&self, method: &str) -> Ident {
        self.exported_fn_name(method)
    }

    /*
    pub fn methods(&self) -> impl Iterator<Item = &'ast Method> {
        self.class
            .items
            .iter()
            .filter_map(|item| match *item {
                ClassItem::Method(ref m) => Some(m),
                _ => None,
            })
    }
    */
}
