# Gnome-class: implement GObjects in Rust with no boilerplate

[GObject][gobject] is the C-based object system for [GTK+][gtk] and
[GNOME][gnome] programs.  While C does not have objects or classes by
itself, GObject makes it possible to write object-oriented C programs.

GObject in C normally requires that you write an uncomfortable amount
of [boilerplate code][boilerplate] to do things like register a new
class, define its methods, register object signals and properties,
etc.  Due to the nature of C, many operations are not type-safe and
depend on correct pointer casts, or on knowing the types that you
should really be passing to varargs functions, which are not checked
by the compiler.

The goal of this Gnome-class crate is to let you write GObject
implementations in Rust with minimal or no boilerplate, and with
compile-time type safety all along.

## How gnome-class works

Gnome-class is a procedural macro for Rust.  Within the macro, we
define a mini-language which looks as Rust-y as possible, and that has
extensions to let you define GObject subclasses, their properties,
signals, interface implementations, and the rest of GObject's
features.  The goal is to require no `unsafe` code on your part.

The procedural macro generates a bunch of code which makes `unsafe`
calls to GObject's C API.  For example, when you write this:

```rust
struct MyPrivateStruct {
    ...
}

gobject_gen! {
    class MyClass: GObject {
        type InstancePrivate = MyPrivateStruct;
    }

    impl MyClass {
        virtual fn my_virtual_method(&self, x: i32) {
            ... do something with x ...
        }
    }
}
```

Then the `gobject_gen!` procedural macro will generate a bunch of code
which both defines a GObject implementation that is callable from
other languages using the [GObject Introspection][gi] machinery, and
which also exports a Rust API using the same conventions
as [glib-rs][glib-rs].

# Goals

* Let users write new GObject classes completely in Rust, with no
  unsafe code, and no boilerplate.

* Generate GObject implementations that look exactly like C GObjects
  from the outside.  The generated GObjects should be callable from C
  or other languages in exactly the same way as traditional GTK+/GNOME
  libraries.

* Automatically emit GObject Introspection information so that the
  generated objects can be consumed by language bindings.

* In the end, we aim to make it compelling for users to *not* write
  new GObject libraries in C, but rather to give them an "obvious" way
  to it in Rust.  This should ensure higher-quality, safer code for
  GNOME's general-purpose libraries, while maintaining backwards
  compatibility with all the GObject-based infrastructure we have.

# Status, and help wanted

As of November 2017, gnome-class is under heavy development, and is
not ready yet for general consumption.  We are not finished
implementing the desired syntax (see the [syntax document][syntax] for
details).  Also, we don't have proposed syntax yet for all the useful
features of GObject.

## Do you want to help with the syntax?

Take a look at the [syntax document][syntax].  Then, read the
"[Internal structure](#internal-structure)" section below to see how
`gobject_gen!` works.

To implement a new syntactic construct, you must first define the
corresponding representation in the AST, and write parser for it.
Then you must hook this new construct into the rest of the analyzed
program in the HIR module.  Finally, you should modify the code
generator to emit Rust code for your new construct as appropriate.

**FIXME:** tour of the code for a specific syntactic feature, from
parsing to code generation.

## Missing features

* Signals.

* Properties.

* Implementing GObject interfaces.

* Defining new GObject interfaces.

* Generate C header files for calling the Rust GObjects.

* Generate GObject Introspection information.

* Allow documentation to be written for the user's GObject API:

  * Generate appropriate Rust documentation for the Rust-side public
    API of the generated objects.

  * Integrate the documentation with the GObject Introspection
    information as the rest of GNOME expects.

# Internal structure {#internal-structure}

The `gobject_gen!` procedural macro works in stages roughly similar to
a compiler:

1. **Parsing.** We parse the code that the user put inside the
   `gobject_gen!` invocation using a [syn][syn]-based parser.  The
   parser generates an **Abstract Syntax Tree** (AST), which closely
   matches the structure of the user's code.  At the end of this
   process, the AST will be fully parsed, but it may not be
   semantically valid.  The AST is defined in `src/ast.rs`.

2. We turn the AST into a **High-level Internal Representation**
   (HIR), which matches GObject concepts more closely.  This is also
   where we ensure that the user's code is semantically valid.  For
   example, we check that there is not more than one `InstancePrivate`
   structure for each class, or that the same signal is not being
   declared twice.  The HIR is defined in `src/hir`.

3. We generate code based on the HIR.  For each class defined in the
   HIR, we emit the necessary GObject boilerplate to register that
   class, its methods, signals, properties, etc.  We emit the actual
   code for methods and signal handlers, and the necessary trampolines
   to call Rust methods and signal handlers from C.  The code
   generator is defined in `src/gen`.  In there, the one-time,
   per-class GObject boilerplate is in `src/gen/boilerplate.rs`.  The
   other files in the `gen` directory are used for things that require
   extra code generation like signals and traits for method trampolines.

The main entry point to the procedural macro is in `src/lib.rs` in the
`gobject_gen` function — note how it has the `#[proc_macro]`
attribute.  This function takes the incoming `TokenStream` from the
Rust compiler, parses it into our AST, creates the HIR from the AST,
and finally calls the code generator upon the HIR.

# Testing

**FIXME:** mention how lib.rs::testme() is a second procedural macro
that actually generates the tests.



[gobject]: https://developer.gnome.org/platform-overview/unstable/tech-gobject.html.en
[boilerplate]: https://developer.gnome.org/SubclassGObject/
[gtk]: https://www.gtk.org/
[gnome]: https://www.gnome.org/
[gi]: https://wiki.gnome.org/Projects/GObjectIntrospection
[glib-rs]: http://gtk-rs.org/docs/glib/
[syntax]: gobject-notes/syntax.md
[syn]: https://github.com/dtolnay/syn/
