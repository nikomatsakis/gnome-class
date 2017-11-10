//use lalrpop_intern::InternedString;
//use quote::Tokens;
use syn::{Ident, Path, FnArg, FunctionRetTy, Block};
use syn::{Attribute, Lit};
use synom::tokens;

pub struct Program {
    pub items: Vec<Item>
}

impl Program {
    pub fn classes<'a>(&'a self) -> impl Iterator<Item = &'a Class> + 'a {
        self.items.iter().filter_map(|item| {
            match *item {
                Item::Class(ref c) => Some(c),
                _ => None,
            }
        })
    }

    pub fn impls<'a>(&'a self) -> impl Iterator<Item = &'a Impl> + 'a {
        self.items.iter().filter_map(|item| {
            match *item {
                Item::Impl(ref i) => Some(i),
                _ => None,
            }
        })
    }
}

pub enum Item {
    Class(Class),
    Impl(Impl),
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
    pub trait_: Option<Ident>,
    pub self_path: Ident,
    pub items: Vec<ImplItem>
}

pub enum ClassItem {
    InstancePrivate(InstancePrivateItem),
}

pub struct ImplItem {
    pub attrs: Vec<Attribute>,
    pub node: ImplItemKind,
}

pub enum ImplItemKind {
    Method(ImplItemMethod),
    ReserveSlots(Lit),
}

pub struct ImplItemMethod {
    pub public: bool, // requires body
    pub virtual_: bool, // implies public, doesn't need body
    pub signal: bool, // ignore
    pub name: Ident,
    pub inputs: Vec<FnArg>, // must start with &self
    pub output: FunctionRetTy,
    pub body: Option<Block>,
}

// Mostly copied from syn's ImplItemType
pub struct InstancePrivateItem {
    pub type_token: tokens::Type,
    pub eq_token: tokens::Eq,
    pub path: Path,
    pub semi_token: tokens::Semi,
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
