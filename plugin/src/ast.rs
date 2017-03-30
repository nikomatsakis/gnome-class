use lalrpop_intern::InternedString;
use quote::Tokens;

pub struct Program {
    pub classes: Vec<Class>
}

pub struct Class {
    pub name: InternedString,
    pub extends: Option<InternedString>,
    pub members: Vec<Member>
}

pub enum Member {
    PrivateStruct(PrivateStruct),
    Init(FnDef),
}

pub struct PrivateStruct {
    pub name: InternedString,
    pub fields: Vec<Field>
}

pub struct FnSig {
    pub args: Vec<Field>,
    pub return_ty: Option<Type>,
}

pub struct FnDef {
    pub sig: FnSig,
    pub code: OpaqueTokens,
}

pub struct Field {
    pub name: InternedString,
    pub ty: Type,
}

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

pub struct OpaqueTokens {
    pub tokens: Tokens,
}
