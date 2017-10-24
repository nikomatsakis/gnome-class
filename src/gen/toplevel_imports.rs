use super::*;

impl<'ast> ClassContext<'ast> {
    pub fn toplevel_imports(&self) -> Tokens {
        quote! {
            extern crate glib_sys as glib_ffi;
            extern crate gobject_sys as gobject_ffi;

            // #[macro_use]
            extern crate glib;

            extern crate libc;

            use glib::{IsA, Value};
            use glib::object::Downcast;
            use glib::signal::connect;
            use glib::translate::*;

            use std::ptr;
            use std::mem;
            use std::mem::transmute;

            // Bring in our parent's stuff so the user's implementation
            // can use what they had already defined there.
            use super::*;

            // #[cfg(feature = "bindings")]
            // mod ffi;

            // #[cfg(feature = "bindings")]
            // pub mod imp {
            //     pub use ffi::*;
            // }
        }
    }
}
