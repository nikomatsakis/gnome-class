use super::*;

impl<'ast> ClassContext<'ast> {
    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32);
    /// ```
    pub fn method_trait_fns(&self) -> Vec<Tokens> {
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
        Vec::new()
    }

    pub fn method_redirects(&self) -> Vec<Tokens> {
        /*
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
         */
        Vec::new()
    }
    /*
    fn method_ffi_name(&self, method: &Method) -> Ident {
        self.exported_fn_name(method.name.as_ref())
    }
     */

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
