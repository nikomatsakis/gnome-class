use lalrpop_intern::InternedString;

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
}

pub struct PrivateStruct {
    pub name: InternedString,
    pub fields: Vec<Field>
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
