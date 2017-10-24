// We give `ClassName` variables an identifier that uses upper-case.
#![allow(non_snake_case)]

use ast::*;
use errors::*;
use quote::{Tokens, ToTokens};
use std::convert::Into;
use syn::Ident;

mod glib_utils;
mod toplevel_imports;
// mod imp;

macro_rules! quote_in {
    ($tokens:expr, $($t:tt)*) => {
        $tokens.append_all(Some(quote!{$($t)*}));
    }
}

// HYGIENE NOTE:
//
// I am using the `__` prefix to indicate names that, while visible
// to the user, are eventually intended to be hidden by hygiene.

pub fn classes(program: &Program) -> Result<Tokens> {
    let class_tokens =
        program.classes
               .iter()
               .map(|class| {
                   let cx = ClassContext::new(program, class);
                   cx.gen_class()
               })
               .collect::<Result<Vec<_>>>()?;
    Ok(quote! { #(#class_tokens)* })
}

struct ClassContext<'ast> {
    program: &'ast Program,
    class: &'ast Class,
    private_struct: &'ast PrivateStruct,
    ModuleName: Ident,
    InstanceName: &'ast Ident,
    ClassName: Ident,
    PrivateClassName: Ident,
    ParentInstance: Tokens,
    ParentInstanceFfi: Tokens,
    ParentClassFfi: Tokens,
    GObject: Tokens,
    GObjectFfi: Tokens,
    GObjectClassFfi: Tokens,
    InstanceExt: Ident,
}

impl<'ast> ClassContext<'ast> {
    pub fn new(program: &'ast Program, class: &'ast Class) -> Self {
        // This function creates a ClassContext by generating the
        // commonly-used symbol names for the class in question, for
        // example, "FooClass" out of "Foo".

        let private_struct =
            class.members
                 .iter()
                 .filter_map(|member| match *member {
                     Member::PrivateStruct(ref ps) => Some(ps),
                     _ => None,
                 })
                 .next();

        let private_struct = match private_struct {
            Some(p) => p,
            None => unreachable!() // this was checked already by checking.rs
        };

        let InstanceName = &class.name;

        // If our instance name is "Foo" and we have a suffix "Bar", generates a "FooBar" Ident.
        // These are used for the generated module name, instance/class struct names, etc.
        macro_rules! container_name {
            ($suffix:expr) => {
                Ident::from(&format!("{}{}", InstanceName.as_ref(), $suffix))
            };
        }

        let ModuleName       = container_name!("Mod"); // toplevel "InstanceMod" module name
        let ClassName        = container_name!("Class");
        let PrivateClassName = container_name!("ClassPrivate");
        let InstanceExt      = container_name!("Ext"); // public trait with all the class's methods

        let GObject         = Self::tokens_GObject();
        let GObjectFfi      = Self::tokens_GObjectFfi();
        let GObjectClassFfi = Self::tokens_GObjectClassFfi();

        let ParentInstance =
            class.extends
                 .as_ref()
                 .map(|c| c.ty())
                 .map(|c| quote! { #c })
                 .unwrap_or_else(|| GObject.clone());
        let ParentInstanceFfi =
            class.extends
                 .as_ref()
                 .map(|c| c.ty())
                 .map(|c| quote! { #c })
                 .unwrap_or_else(|| GObjectFfi.clone());
        let ParentClassFfi = quote! {
            <#ParentInstance as glib::wrapper::Wrapper>::GlibClassType
        };

        ClassContext {
            program,
            class,
            private_struct,
            ModuleName,
            InstanceName,
            ClassName,
            PrivateClassName,
            ParentInstance,
            ParentInstanceFfi,
            ParentClassFfi,
            GObject,
            GObjectFfi,
            GObjectClassFfi,
            InstanceExt,
        }
    }

    pub fn gen_class(&self) -> Result<Tokens> {
        let all = vec![
            self.toplevel_imports(),
            self.glib_wrapper(),
/*
            self.imp_module(),
            self.pub_impl(),
            self.instance_ext(),
            self.instance_ext_impl(),
            self.signal_trampolines(),
*/
        ];

        let ModuleName = &self.ModuleName;

        Ok(quote! {
            pub mod #ModuleName {
                #(#all)*
            }

            pub use #ModuleName::*;
        })
    }

    fn callback_guard(&self) -> Tokens {
        quote! {
            let _guard = glib::CallbackGuard::new();
        }
    }

    fn exported_fn_name(&self, method_name: &str) -> Ident {
        Ident::from(&format!("{}_{}", self.lower_case_class_name(), method_name))
    }

    fn get_type_fn_name(&self) -> Ident {
        self.exported_fn_name("get_type")
    }

    fn pub_impl(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let pub_new_method = self.pub_new_method();

        quote! {
            impl #InstanceName {
                #pub_new_method
            }
        }
    }

    fn pub_new_method(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let imp_new_fn_name = self.imp_new_fn_name();

        // FIXME: we should take construct-only arguments and other convenient args to new()
        quote! {
            pub fn new() -> #InstanceName {
                unsafe { from_glib_full(imp::#imp_new_fn_name(/* FIXME: args */)) }
            }
        }
    }

    fn imp_new_fn_name(&self) -> Ident {
        self.exported_fn_name("new")
    }

    fn instance_ext(&self) -> Tokens {
        let InstanceExt = self.InstanceExt;
        let method_trait_fns = &self.method_trait_fns();

        quote! {
            pub trait #InstanceExt {
                #(#method_trait_fns)*

                // FIXME: property setters/getters like in glib-rs
                //
                // fn get_property_foo(&self) -> type;
                //
                // fn set_property_foo(&self, v: type);

                // FIXME: methods to connect to signals like in glib-rs
                //
                // fn connect_signalname<F: Fn(&Self, type, type) -> type + 'static>(&self, f: F) -> u64;
            }
        }
    }

    fn instance_ext_impl(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let InstanceExt = self.InstanceExt;
        let method_redirects = self.method_redirects();
        quote! {
            impl<O: IsA<#InstanceName> + IsA<glib::object::Object>> #InstanceExt for O {
                #(#method_redirects)*

                // FIXME: property setters/getters like in glib-rs
                //
                // fn get_property_foo(&self) -> type {
                //     let mut value = Value::from(&false); // FIXME: Value::from(&what)?
                //     unsafe {
                //         gobject_ffi:g_object_get_property(self.to_glib_none().0, "foo".to_glib_none().0, value.to_glib_none_mut().0);
                //     }
                //     value.get().unwrap()
                // }
                //
                // fn set_property_foo(&self, v: type) {
                //     unsafe {
                //         gobject_ffi:g_object_set_property(self.to_glib_none().0, "foo".to_glib_none().0, Value::from(&v).to_glib_none().0);
                //     }
                // }

                // FIXME: methods to connect to signals like in glib-rs
                //
                // fn connect_signalname<F: Fn(&Self, type, type) -> type + 'static>(&self, f: F) -> u64 {
                //     unsafe {
                //         let f: Box_<Box_<Fn(&Self, type, type) -> type + 'static>> = Box_::new(Box_::new(f));
                //         connect(self.to_glib_none().0, "signalname",
                //             transmute(signalname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
                //     }
                // }
            }
        }
    }

    fn signal_trampolines(&self) -> Tokens {
        let InstanceName = self.InstanceName;
        let InstanceExt = self.InstanceExt;

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

    pub fn methods(&self) -> impl Iterator<Item = &'ast Method> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(ref m) => Some(m),
                _ => None,
            })
    }

    pub fn signals(&self) -> impl Iterator<Item = &'ast Signal> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Signal(ref s) => Some(s),
                _ => None,
            })
    }

    pub fn members_with_slots(&self) -> impl Iterator<Item = &'ast Member> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(_) => Some(member),
                Member::Signal(_) => Some(member),
                _ => None,
            })
    }

    fn slot_trampoline_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_trampoline", slot_name.as_ref()))
    }

    fn slot_impl_name(slot_name: &Ident) -> Ident {
        Ident::from(&format!("{}_impl", slot_name.as_ref()))
    }

    fn slot_assignments(&self) -> Vec<Tokens> {
        let InstanceName = self.InstanceName;

        self.members_with_slots()
            .map(|member| {
                let slot_name = match *member {
                    Member::Method(ref method) => method.name,
                    Member::Signal(ref signal) => signal.name,
                    _ => unreachable!()
                };

                let trampoline_name = Self::slot_trampoline_name(&slot_name);

                quote! {
                    klass.#slot_name = Some(#InstanceName::#trampoline_name);
                }
            })
            .collect()
    }

    fn signal_id_name(signal: &Signal) -> Ident {
        Ident::from(&format!("{}_signal_id", signal.name.as_ref()))
    }

    /// From a signal called `foo`, generate `foo_signal_id`.  This is used to
    /// store the signal ids from g_signal_newv() in the Class structure.
    pub fn signal_id_names(&self) -> Vec<Ident> {
        self.signals()
            .map(|signal| signal_id_name (signal))
            .collect()
    }

    pub fn method_names(&self) -> Vec<Ident> {
        self.methods()
            .map(|method| method.name)
            .collect()
    }

    fn signal_declarations(&self) -> Vec<Tokens> {
        self.signals()
            .map(|signal| {
                // FIXME: we are not specifying the proper signature (return, args) for the signal
                // handler.  We need a way to translate Rust type names into G_TYPE_* ids.
                //
                // FIXME: we are not passing signal flags
                //
                // FIXME: We are not passing a class_closure, marshaller, etc.

                let signal_id_name = signal_id_name(&signal);
                let signal_name = ByteString(signal.name);
                quote! {
                    klass.#signal_id_name =
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
    }

    pub fn method_fn_tys(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let method_fn_ty = SlotTy {
                    class_name: self.InstanceName,
                    sig: &method.fn_def.sig
                };
                quote! { #method_fn_ty }
            })
            .collect()
    }

    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32);
    /// ```
    pub fn method_trait_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty;
                }
            })
            .collect()
    }

    fn method_redirects(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let ffi_name = self.method_ffi_name(method);
                let arg_decls = method.fn_def.sig.arg_decls();
                let arg_names = method.fn_def.sig.arg_names();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty {
                        unsafe { imp::#ffi_name(self.to_glib_none().0, #arg_names) }
                    }
                }
            })
            .collect()
    }

    fn method_ffi_name(&self, method: &Method) -> Ident {
        self.exported_fn_name(method.name.as_ref())
    }

    fn lower_case_class_name(&self) -> String {
        lalrpop_intern::read(|interner| {
            let name_str = interner.data(self.InstanceName.as_ref());
            let mut name_chars = name_str.chars();
            let first_char: char = name_chars.next().unwrap();
            first_char.to_lowercase().chain(name_chars).collect()
        })
    }
}

impl ToTokens for VarTy {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &VarTy { name, ref ty } = self;
        quote_in!(tokens, #name: #ty)
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Type::Path(ref path) => {
                path.ty().to_tokens(tokens)
            }
            Type::Array(ref ty) => {
                quote_in!(tokens, [ #ty ]);
            }
            Type::Sum(ref tys) => {
                quote_in!(tokens, #(#tys)+*);
            }
        }
    }
}

struct ByteString(Ident);

impl ToTokens for ByteString {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append("b\"");
        tokens.append(self.0.as_ref());
        tokens.append("\\0\"");
    }
}

impl ToTokens for CodeBlock {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.tokens.to_tokens(tokens)
    }
}

struct SlotTy<'ast> {
    class_name: Ident,
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for SlotTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let class_name = self.class_name;
        let arg_decls = self.sig.arg_decls();
        let return_ty = self.sig.return_ty();

        quote_in! {
            tokens,
            (this: *mut #class_name, #arg_decls) #return_ty
        }
    }
}

struct SlotImplTy<'ast> {
    sig:  &'ast FnSig,
}

impl<'ast> ToTokens for SlotImplTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let arg_decls = self.sig.arg_decls();
        let return_ty = self.sig.return_ty();

        quote_in! {
            tokens,
            (&self, #arg_decls) #return_ty
        }
    }
}


/// Helper methods for printing out various bits of
/// a method signature. For each of the comments below,
/// assume an example method `fn get(&self, x: u32, y: u32) -> u32`.
impl FnSig {
    /// Generates `x: u32, y: u32`
    fn arg_decls<'ast>(&'ast self) -> ArgDecls<'ast> {
        ArgDecls { sig: self }
    }

    /// Generates `x, y`
    fn arg_names<'ast>(&'ast self) -> ArgNames<'ast> {
        ArgNames { sig: self }
    }

    /// Generates `-> u32` (or `` if unit)
    fn return_ty<'ast>(&'ast self) -> ReturnTy<'ast> {
        ReturnTy { sig: self }
    }
}

struct ArgDecls<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgDecls<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = &self.sig.args;
        quote_in!(tokens, #(#args),*);
    }
}

struct ArgNames<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgNames<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = self.sig.args.iter().map(|arg| arg.name);
        quote_in!(tokens, #(#args),*);
    }
}

struct ReturnTy<'ast> {
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for ReturnTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if let Some(ref return_ty) = self.sig.return_ty {
            quote_in!(tokens, -> #return_ty)
        }
    }
}

impl Path {
    fn ty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: false, path: self }
    }

    fn exprty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: true, path: self }
    }
}

struct SepPath<'a> {
    cc: bool,
    path: &'a Path,
}

impl<'a> ToTokens for SepPath<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self.path {
            Path::FromRoot => tokens.append("::"),
            Path::FromSelf => tokens.append("self"),
            Path::FromSuper => tokens.append("super"),
            Path::FromTraitItem(ref i) => i.to_tokens(tokens),
            Path::From(ref i) => {
                let i = SepPathId { cc: self.cc, path_id: i};
                i.to_tokens(tokens)
            }
            Path::Extend(ref b, ref i) => {
                let b = SepPath { cc: self.cc, path: b };
                let i = SepPathId { cc: self.cc, path_id: i };
                quote_in!(tokens, #b :: #i)
            }
        }
    }
}

struct SepPathId<'a> {
    cc: bool,
    path_id: &'a PathId,
}

impl<'a> ToTokens for SepPathId<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.path_id.name.to_tokens(tokens);
        let tys = &self.path_id.tys;
        if !tys.is_empty() {
            if self.cc {
                tokens.append("::");
            }
            quote_in!(tokens, <#(#tys),*>);
        }
    }
}

impl ToTokens for TraitItemId {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &TraitItemId { ref self_ty, ref trait_ref, item } = self;
        let trait_ref = trait_ref.ty();
        quote_in!(tokens, < #self_ty as #trait_ref > :: #item);
    }
}
