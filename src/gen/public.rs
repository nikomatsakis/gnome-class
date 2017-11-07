use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn pub_impl(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let pub_new_method = self.pub_new_method();

        quote! {
            impl #InstanceName {
                #pub_new_method
            }
        }
    }

    fn pub_new_method(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let imp_new_fn_name = self.imp_new_fn_name();

        // FIXME: we should take construct-only arguments and other convenient args to new()
        quote! {
            pub fn new() -> #InstanceName {
                unsafe { from_glib_full(imp::#imp_new_fn_name(/* FIXME: args */)) }
            }
        }
    }

}
