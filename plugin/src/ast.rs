use lalrpop_intern::InternedString;
use quote::Tokens;

#[derive(Debug)]
pub struct Program {
    pub classes: Vec<Class>
}

#[derive(Debug)]
pub struct Class {
    pub name: Identifier,
    pub extends: Option<Path>,
    pub members: Vec<Member>
}

#[derive(Debug)]
pub enum Member {
    PrivateStruct(PrivateStruct),
    Init(CodeBlock),
    Method(Method),
}

#[derive(Debug)]
pub struct PrivateStruct {
    pub name: Identifier,
    pub fields: Vec<VarTy>
}

#[derive(Debug)]
pub struct FnSig {
    pub args: Vec<VarTy>,
    pub return_ty: Option<Type>,
}

#[derive(Debug)]
pub struct Method {
    pub name: Identifier,
    pub fn_def: FnDef,
}

#[derive(Debug)]
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
#[derive(Debug)]
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
    Extend(Box<Path>, PathId)
}

#[derive(Debug)]
pub struct PathId {
    pub name: Identifier,
    pub tys: Vec<Type>
}
