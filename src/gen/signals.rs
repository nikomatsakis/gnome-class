use super::*;
use self::cstringident::*;

impl<'ast> ClassContext<'ast> {
    pub fn signal_trampolines(&self) -> Tokens {
        quote! {
            // FIXME: signal handler trampolines like in glib-rs
            //
            // unsafe extern "C" fn signalname_trampoline<P>(this: *mut ffi::InstanceName, argname: type, argname: type, f: glib_ffi:gpointer) -> type
            // where P: IsA<InstanceName> {
            //     callback_guard!();
            //     let f: &&(Fn(&P, type, type) -> type + 'static) = transmute(f);
            //
            //     // with return value:
            //     f(&InstanceName::from_glib_none(this).downcast_unchecked(), &from_glib_none(argname), &from_glib_none(argname))).to_glib()
            //
            //     // without return value:
            //     f(&InstanceName::from_glib_none(this).downcast_unchecked(), &from_glib_none(argname), &from_glib_none(argname)))
            //
            //     // those are by-reference arguments.  For by-value arguments,
            //     from_glib(argname)
            // }
        }
    }

    pub fn signal_declarations(&self) -> Vec<Tokens> {
        /*
        self.signals()
            .map(|signal| {
                // FIXME: we are not specifying the proper signature (return, args) for the signal
                // handler.  We need a way to translate Rust type names into G_TYPE_* ids.
                //
                // FIXME: we are not passing signal flags
                //
                // FIXME: We are not passing a class_closure, marshaller, etc.

                let signal_id_name = signal_id_name(&signal);
                let signal_name = CStringIdent(signal.name);
                quote! {
                    PRIV.signals[signal_id_name] =
                        gobject_sys::g_signal_newv (#signal_name as *const u8 as *const i8,
                                                    (*g_object_class).g_type_class.g_type,
                                                    gobject_sys::G_SIGNAL_RUN_FIRST, // flags
                                                    ptr::null_mut(),                 // class_closure,
                                                    None,                            // accumulator
                                                    ptr::null_mut(),                 // accu_data
                                                    None,                            // c_marshaller,
                                                    gobject_sys::G_TYPE_NONE,        // return_type
                                                    0,                               // n_params,
                                                    ptr::null_mut()                  // param_types
                        );
                }
            })
            .collect()
         */
        Vec::new()
    }

    /*
    /// From a signal called `foo`, generate `foo_signal_id`.  This is used to
    /// store the signal ids from g_signal_newv() in the Class structure.
    fn signal_id_name(signal: &Signal) -> Ident {
        Ident::from(&format!("{}_signal_id", signal.name.as_ref()))
    }
    */

    pub fn signal_id_names(&self) -> Vec<Ident> {
        /*
        self.signals()
            .map(|signal| signal_id_name (signal))
            .collect()
         */
        Vec::new()
    }

    /*
    pub fn signals(&self) -> impl Iterator<Item = &'ast Signal> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Signal(ref s) => Some(s),
                _ => None,
            })
    }
    */
}
