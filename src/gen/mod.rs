// We give `ClassName` variables an identifier that uses upper-case.
#![allow(non_snake_case)]

use ast::*;
use errors::*;
use quote::{Tokens};
use std::convert::Into;
use syn::Ident;

mod glib_utils;
use self::glib_utils::lower_case_instance_name;

mod toplevel_imports;
// mod imp;
// mod pub;
// mod instance_ext;
// mod signals;

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
                Ident::from(format!("{}{}", InstanceName.as_ref(), $suffix))
            };
        }

        let ModuleName       = container_name!("Mod"); // toplevel "InstanceMod" module name
        let ClassName        = container_name!("Class");
        let PrivateClassName = container_name!("ClassPrivate");
        let InstanceExt      = container_name!("Ext"); // public trait with all the class's methods

        let GObject          = Self::tokens_GObject();
        let GObjectFfi       = Self::tokens_GObjectFfi();
        let GObjectClassFfi  = Self::tokens_GObjectClassFfi();

        let ParentInstance    = Self::tokens_ParentInstance(class);
        let ParentInstanceFfi = Self::tokens_ParentInstanceFfi(class);
        let ParentClassFfi    = Self::tokens_ParentClassFfi(class);

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

    fn exported_fn_name(&self, method_name: &str) -> Ident {
        Ident::from(format!("{}_{}", lower_case_instance_name(self.InstanceName.as_ref()), method_name))
    }

    fn instance_get_type_fn_name(&self) -> Ident {
        self.exported_fn_name("get_type")
    }
}

/*
use quote::{ToTokens};


macro_rules! quote_in {
    ($tokens:expr, $($t:tt)*) => {
        $tokens.append_all(Some(quote!{$($t)*}));
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
*/
