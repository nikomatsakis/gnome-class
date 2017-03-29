#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use] extern crate lazy_static;
#[macro_use] mod macros;

extern crate gobject_sys;
extern crate glib_sys;

mod g;
mod gobject;
mod ptr;
mod mock;
mod real;

pub mod prelude {
    pub use ptr::Ptr;
}
