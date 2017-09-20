#![feature(catch_expr)]
#![feature(conservative_impl_trait)]
#![feature(proc_macro)]
#![recursion_limit="256"]

// While under active devel, these warnings are kind of annoying.
#![allow(dead_code)]

#[macro_use] extern crate error_chain;
extern crate lalrpop_intern;
extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate unicode_xid;
extern crate rustfmt;

use proc_macro::TokenStream;
use errors::*;
use std::str::FromStr;

mod ast;
mod errors;
mod gen;
mod param;
mod parser;
mod tok;

/// Generates the code to create a derived glib::Object
///
/// This procedural macro defines an extension to the Rust language so
/// that one can create GObjects using only safe code.  All the
/// boilerplate needed to register the GObject type, its signals and
/// properties, etc., is automatically generated.
///
/// # Syntax overview {#syntax-overview}
///
/// The macro is invoked as follows:
///
/// ```ignore
/// #[macro_use]
/// extern crate glib;  // see "Necessary imports" below on why this is needed
///
/// gobject_gen! {
///     class Foo {
///         struct FooPrivate {
///             // ... your instance data here
///         }
///
///         // Optional private_init() function, see below
///
///         // Methods and signals;, their order defines the ABI of your class
///     }
/// }
/// ```
///
/// # Private structure and the optional `private_init()` function
///
/// All GObject classes defined through this macro must have a
/// declaration for a private structure, which is used for each
/// instance's private data.
///
/// Within the macro invocation, you can declare the private structure
/// as for `FooPrivate` in the [syntax overview][syntax-overview] above.
///
/// If you don't do anything else, all the fields in your `FooPrivate` structure
/// will be initialized to `Default::default()` — this implies that all the types 
/// of your struct's fields must implement the `Default` trait.
///
/// Alternatively, you can define a special `private_init()` function
/// that will be used to initialize your private structure from custom
/// values.  This function must return a value of the same type as
/// your private structure; this value will be used by the GObject
/// system to initialize your `FooPrivate`:
///
/// ```ignore
/// gobject_gen! {
///     class FooWithCustomInit {
///         struct FooPrivate {
///             bar: SomeType,
///             baz: SomeOtherType,
///         }
///
///         private_init() -> FooPrivate {
///             // Provide the initial value of FooPrivate as our return value
///             FooPrivate {
///                 bar: SomeType::new(...),
///                 baz: SomeOtherType::new(...),
///             }
///         }
///     }
/// }
/// ```
///
/// **Note**: the `private_init()` function does not take `&self`.
/// Its only purpose is to provide an initial value for your private
/// structure.  At the point which `private_init()` is run, your
/// `FooWithCustomInit` instance is not even fully initialized nor
/// constructed.  From `private_init()`, just return a `FooPrivate`
/// that is suitable for your instance's initial state.
///
/// [syntax-overview]: #syntax-overview
///
/// # ABI considerations
///
/// FIXME
///
/// # Declaring methods
///
/// FIXME
///
/// # Declaring signals
///
/// FIXME
///
/// # Example: simple class derived from glib::Object
///
/// ```ignore
/// #[macro_use]
/// extern crate glib;  // see "Necessary imports" below on why this is needed
///
/// use std::cell::Cell;
///
/// gobject_gen! {
///     class Foo {
///         struct FooPrivate {
///             val: Cell<u32>
///         }
///
///         // FIXME: continue the documentation
///     }
/// }
/// ```
///
/// # Necessary imports
///
/// The generated code depends on external crates which you must put in your `Cargo.toml`:
///
/// * The `libc` crate
/// * The `glib` crate and its macros
///
/// At the top of your crate's main file, you must declare macro use
/// for the `glib` crate:
///
/// ```ignore
/// #[macro_use]
/// extern crate glib;
/// ```
///
#[proc_macro]
pub fn gobject_gen(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let result: Result<quote::Tokens> = do catch {
        let program = parser::parse_program(&input)?;
        gen::classes(&program)
    };

    match result {
        Ok(token_stream) => {
            let mut config: rustfmt::config::Config = Default::default();
            let mut out: Vec<u8> = vec!();
            config.set().write_mode(rustfmt::config::WriteMode::Plain);
            config.set().error_on_line_overflow(false);
            let stream: String = token_stream.as_str().into();
            match rustfmt::format_input(rustfmt::Input::Text(stream),
                                        & config,
                                        Some(& mut out)) {
                Ok(_) => {
                    let output = String::from_utf8(out).unwrap();
                    println!("/********************************************************************************/\n{}", output);
                    TokenStream::from_str(& output).unwrap()
                },
                Err(e) => {
                    println!("{}", e.0);
                    panic!("cannot generate gobjects")
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
            panic!("cannot generate gobjects")
        }
    }
}
