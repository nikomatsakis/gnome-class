#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use] mod macros;

mod gobject;
mod ptr;
mod mock;
mod real;

pub mod prelude {
    pub use ptr::Ptr;
}
