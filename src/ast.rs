//use lalrpop_intern::InternedString;
//use quote::Tokens;
use syn::{DeriveInput, Ident, ImplItem, Path, FnArg, FunctionRetTy, Block};
use synom::delimited::Delimited;
use synom::tokens;

pub struct Program {
    pub items: Vec<Item>
}

pub enum Item {
    Class(Class),
    Impl(Impl)
}

pub fn get_program_classes<'a>(program: &'a Program) -> Vec<&'a Class> {
    program.items
        .iter()
        .filter_map(|item| {
            if let Item::Class(ref c) = *item {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

pub struct Class {
    pub name: Ident,
    pub extends: Option<Path>,
    pub items: Vec<ClassItem>
}

// similar to syn::ItemImpl
pub struct Impl {
    pub impl_token: tokens::Impl,
    pub trait_: Option<(Path, tokens::For)>,
    pub self_path: Path,
    pub brace_token: tokens::Brace,
    pub items: Vec<ImplItem>
}

pub enum ClassItem {
    InstancePrivate(InstancePrivateItem),
    PrivateStruct(PrivateStruct),
    PrivateInit(PrivateInit),
/*
    Method(Method),
    Signal(Signal),
*/
}

// Mostly copied from syn's ImplItemType
pub struct InstancePrivateItem {
    pub type_token: tokens::Type,
    pub eq_token: tokens::Eq,
    pub path: Path,
    pub semi_token: tokens::Semi,
}

pub struct PrivateStruct {
    pub derive_input: DeriveInput
}

pub struct PrivateInit {
    // FIXME: inputs must be empty; we just don't know how to parse
    // an empty "()" yet.
    pub inputs: Delimited<FnArg, tokens::Comma>,

    // checked to be the same as the PrivateStruct type
    pub output: FunctionRetTy,

    pub block: Block
}
/*
pub struct Signal {
    pub name: Ident,
    pub sig: FnSig,
    pub code: Option<CodeBlock>,
    // FIXME: signal flags
}

pub struct FnSig {
    pub args: Vec<VarTy>,
    pub return_ty: Option<Type>,
}

pub struct Method {
    pub name: Ident,
    pub fn_def: FnDef,
}

pub struct FnDef {
    pub sig: FnSig,
    pub code: CodeBlock,
}

pub struct VarTy {
    pub name: Ident,
    pub ty: Type,
}

pub enum Type {
    // N
    Path(Path),

    // [N]
    Array(Box<Type>),

    // N + ... + N
    Sum(Vec<Type>),
}

/// Some uninterpreted tokens that always begin/end with `{`/`}`.
pub struct CodeBlock {
    pub tokens: Tokens,
}
*/
/*
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub str: InternedString
}

#[derive(Debug)]
pub enum Path {
    FromRoot,
    FromSelf,
    FromSuper,
    From(PathId),
    FromTraitItem(Box<TraitItemId>),
    Extend(Box<Path>, PathId)
}

#[derive(Debug)]
pub struct PathId {
    pub name: Ident,
    pub tys: Vec<Type>
}

#[derive(Debug)]
pub struct TraitItemId {
    pub self_ty: Type,
    pub trait_ref: Path,
    pub item: Ident,
}
*/

impl PrivateStruct {
    pub fn name(&self) -> &Ident {
        &self.derive_input.ident
    }

    pub fn name_as_ref(&self) -> &str {
        self.derive_input.ident.as_ref()
    }
}
