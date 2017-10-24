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

    pub fn tokens_GObject() -> Tokens {
        quote! { glib::Object }
    }

    pub fn tokens_GObjectFfi() -> Tokens {
        quote! { gobject_ffi::GObject }
    }

    pub fn tokens_GObjectClassFfi() -> Tokens {
        quote! { gobject_ffi::GObjectClass }
    }
}
