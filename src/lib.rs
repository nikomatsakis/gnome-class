#![feature(catch_expr)]
#![feature(conservative_impl_trait)]
#![feature(proc_macro)]
#![recursion_limit="512"]

// While under active devel, these warnings are kind of annoying.
#![allow(dead_code)]

#[macro_use] extern crate error_chain;
// extern crate lalrpop_intern;
// extern crate lalrpop_util;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate unicode_xid;
extern crate rustfmt;

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use errors::*;

macro_rules! quote_cs {
    ($($tt:tt)*) => (quote_spanned!(::proc_macro2::Span::call_site()=>
                                    $($tt)*))
}

mod ast;
mod checking;
mod errors;
mod gen;
mod glib_utils;
mod hir;
mod param;
mod parser;

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
/// use gobject_gen::gobject_gen;
///
/// struct FooPrivate {
///     // ... your instance data here
/// }
///
/// impl Default for FooPrivate {
///     fn default() -> FooPrivate {
///         // initialize your instance data here
///     }
/// }
///
/// gobject_gen! {
///     class Foo {
///         type InstancePrivate = FooPrivate;
///     }
///
///     // Methods and signals;, their order defines the ABI of your class
///     impl Foo {
///         pub fn a_static_method(&self) {
///             // self.get_priv() gives us a &mut FooPrivate - this is how
///             // you access your instance-private data
///             do_something_with(self.get_priv());
///         }
///
///         virtual fn a_virtual_method(&self) {
///             // default handler implementation goes here
///         }
/// }
/// ```
///
/// Read on for the details on how to use GObject features.
///
/// # Instance-private data
///
/// GObject classes defined through this macro can have an optional
/// instance-private data structure.  To specify instance-private data:
///
/// * **Declaration:** Declare a `struct` to contain your data outside
/// of the `gobject_gen!` invocation.
///
/// * **Initialization:** Implement the `Default` trait for your
/// struct, either with `#[derive(Default)]` if those values suit your
/// purposes, or with an `impl Default` if you need custom values.
/// When the generated code needs to initialize the instance-private
/// data, it will do so by calling your struct's `::default()` method.
///
/// * **Bind to the class:** Specify `type InstancePrivate =
/// MyPrivateStruct;` inside the `class` item in the `gobject_gen!`
/// invocation.
///
/// * **Drop:** When the GObject instance gets finalized, your private
/// data will be `drop()`ed.  You can provide `impl Drop` for any fields
/// that need explicit resource management.
///
/// ## Example: instance-private data with default values
///
/// ```ignore
/// #[derive(Default)]
/// struct FooPrivate {
///     // ... your fields here
///     //
///     // They will get initialized per #[derive(Default)]
/// }
///
/// gobject_gen! {
///     class Foo {
///         type InstancePrivate = FooPrivate;
///         ...
///     }
/// }
/// ```
///
/// ## Example: instance-private data with custom values
///
/// ```ignore
/// struct FooPrivate {
///     value: Cell<i32>;
/// }
///
/// impl Default for FooPrivate {
///     fn default() -> FooPrivate {
///         FooPrivate {
///             value: Cell::new(42); // note our custom initial value here
///         }
///     }
/// }
///
/// gobject_gen! {
///     class Foo {
///         type InstancePrivate = FooPrivate;
///         ...
///     }
/// }
/// ```
///
/// # Declaring methods
///
/// FIXME
///
/// # Declaring signals
///
/// FIXME
///
/// # ABI considerations
///
/// FIXME
///
/// # Necessary imports
///
/// The generated code depends on external crates which you must put in your `Cargo.toml`:
///
/// * The `glib` crate and its macros.
/// * The `gobject_gen` crate, declaring `proc_macro` use.
///
/// You can put this at the top of your crate's main file:
///
/// ```ignore
/// #![feature(proc_macro)]
/// extern crate gobject_gen;
///
/// #[macro_use]
/// extern crate glib;
///
/// use gobject_gen::gobject_gen;
/// ```
///
#[proc_macro]
pub fn gobject_gen(input: TokenStream) -> TokenStream {

    let result: Result<quote::Tokens> = do catch {
        let ast_program = parser::parse_program(input)?;
        let program = hir::Program::from_ast_program(&ast_program)?;
        gen::classes(&program)
    };

    match result {
        Ok(tokens) => {
            let mut config: rustfmt::config::Config = Default::default();
            let mut out: Vec<u8> = vec!();
            config.set().write_mode(rustfmt::config::WriteMode::Plain);
            config.set().error_on_line_overflow(false);
            let stream: String = tokens.to_string().into();
            match rustfmt::format_input(rustfmt::Input::Text(stream),
                                        & config,
                                        Some(& mut out)) {
                Ok(_) => {
                    let output = String::from_utf8(out).unwrap();
                    println!("/********************************************************************************/\n{}", output);
                    tokens.into()
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

#[proc_macro]
pub fn testme(input: TokenStream) -> TokenStream {
    checking::tests::run();
    glib_utils::tests::run();
    hir::tests::run();
    parser::tests::run();
    return input
}
