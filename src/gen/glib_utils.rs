use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn glib_wrapper(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let get_type_fn_name = self.instance_get_type_fn_name();

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

    fn glib_callback_guard(&self) -> Tokens {
        quote! {
            let _guard = glib::CallbackGuard::new();
        }
    }
}

pub fn lower_case_instance_name(instance_name: &str) -> String {
    let mut name_chars = instance_name.chars();
    let first_char: char = name_chars.next().unwrap();
    first_char.to_lowercase().chain(name_chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_cases_simple_names() {
        assert_eq!("foo", lower_case_instance_name("Foo"));
    }
}
