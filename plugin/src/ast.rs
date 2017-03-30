use lalrpop_intern::InternedString;
use quote::Tokens;

#[derive(Debug)]
pub struct Program {
    pub classes: Vec<Class>
}

#[derive(Debug)]
pub struct Class {
    pub name: InternedString,
    pub extends: Option<InternedString>,
    pub members: Vec<Member>
}

#[derive(Debug)]
pub enum Member {
    PrivateStruct(PrivateStruct),
    Init(FnDef),
}

#[derive(Debug)]
pub struct PrivateStruct {
    pub name: InternedString,
    pub fields: Vec<Field>
}

#[derive(Debug)]
pub struct FnSig {
    pub args: Vec<Field>,
    pub return_ty: Option<Type>,
}

#[derive(Debug)]
pub struct FnDef {
    pub sig: FnSig,
    pub code: OpaqueTokens,
}

#[derive(Debug)]
pub struct Field {
    pub name: InternedString,
    pub ty: Type,
}

#[derive(Debug)]
pub enum Type {
    // N
    Name(InternedString),

    // N<>
    Args(InternedString, Vec<Type>),

    // [N]
    Array(Box<Type>),

    // N + ... + N
    Sum(Vec<Type>),
}

#[derive(Debug)]
pub struct OpaqueTokens {
    pub tokens: Tokens,
}
