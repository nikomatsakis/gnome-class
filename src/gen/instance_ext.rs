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
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { public, name, inputs, output, body: _ }) => {
                        drop(public); // TODO: use this?
                        quote! {
                            fn #name(#(#inputs),*) #output;
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
                    Slot::Signal(_) => panic!("signals not implemented"),
                }
            })
            .collect()
        /*
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
         */
        // Vec::new()
    }

    pub fn method_redirects(&self) -> Vec<Tokens> {
        self.class
            .slots
            .iter()
            .map(|slot| {
                match *slot {
                    Slot::Method(Method { public, name, inputs, output, body: _ }) => {
                        let ffi_name = self.method_ffi_name(name.sym.as_str());
                        let arg_names = ArgNames(&inputs[1..]);
                        drop(public); // TODO: use this?
                        quote! {
                            fn #name(#(#inputs),*) #output {
                                unsafe {
                                    imp::#ffi_name(self.to_glib_none().0,
                                                   #arg_names)
                                }
                            }
                        }
                    }
                    Slot::VirtualMethod(_) => panic!("virtual methods not implemented"),
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
