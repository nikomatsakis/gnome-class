use lalrpop_intern::InternedString;

pub struct Program {
    classes: Vec<Class>
}

pub struct Class {
    name: Id,
    extends: Id,
    members: Vec<Member>
}

pub enum Member {
    PrivateStruct(PrivateStruct),
}

pub struct PrivateStruct {
    name: InternedString,
    fields: Vec<Field>
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
