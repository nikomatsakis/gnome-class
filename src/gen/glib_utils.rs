use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn glib_wrapper(&self) -> Tokens {
        let InstanceName = self.InstanceName;
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
}
