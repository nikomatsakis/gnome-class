//use lalrpop_intern::InternedString;
use quote::Tokens;
use syn::{Ident, Path};

pub struct Program {
    pub classes: Vec<Class>
}

pub struct Class {
    pub name: Ident,
    pub extends: Option<Path>,
    pub members: Vec<Member>
}

pub enum Member {
    PrivateStruct(PrivateStruct),
    PrivateInit(CodeBlock),
    Method(Method),
    Signal(Signal),
}

pub struct PrivateStruct {
    pub name: Ident,
    pub fields: Vec<VarTy>
}

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
