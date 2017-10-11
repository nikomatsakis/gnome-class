use lalrpop_intern::InternedString;
use quote::Tokens;

pub struct Program {
    pub classes: Vec<Class>
}

pub struct Class {
    pub name: Identifier,
    pub extends: Option<Path>,
    pub members: Vec<Member>
}

pub enum Member {
    PrivateStruct(PrivateStruct),
    PrivateInit(CodeBlock),
    Method(Method),
    Signal(Signal),
}

#[derive(Debug)]
pub struct PrivateStruct {
    pub name: Identifier,
    pub fields: Vec<VarTy>
}

pub struct Signal {
    pub name: Identifier,
    pub sig: FnSig,
    pub code: Option<CodeBlock>,
    // FIXME: signal flags
}

#[derive(Debug)]
pub struct FnSig {
    pub args: Vec<VarTy>,
    pub return_ty: Option<Type>,
}

pub struct Method {
    pub name: Identifier,
    pub fn_def: FnDef,
}

pub struct FnDef {
    pub sig: FnSig,
    pub code: CodeBlock,
}

#[derive(Debug)]
pub struct VarTy {
    pub name: Identifier,
    pub ty: Type,
}

#[derive(Debug)]
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
    pub name: Identifier,
    pub tys: Vec<Type>
}

#[derive(Debug)]
pub struct TraitItemId {
    pub self_ty: Type,
    pub trait_ref: Path,
    pub item: Identifier,
}
