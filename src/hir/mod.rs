// High-level Internal Representation of GObject artifacts
//
// Here we provide a view of the world in terms of what GObject knows:
// classes, interfaces, signals, etc.
//
// We construct this view of the world from the raw Abstract Syntax
// Tree (AST) from the previous stage.

use syn::{Ident, ImplItem, Path};

use super::ast;

pub struct Program<'ast> {
    pub classes: Vec<Class<'ast>>
}

pub struct Class<'ast> {
    pub name: Ident,
    pub superclass: Path,
    pub implements: Vec<Path>, // names of GTypeInterfaces

    pub instance_private: Option<&'ast ast::PrivateStruct>,
    // pub class_private: Option<&'ast ast::PrivateStruct>

    // The order of these is important; it's the order of the slots in FooClass
    pub slots: Vec<Slot>,
    // pub n_reserved_slots: usize,

    // pub properties: Vec<Property>,
}

pub enum Slot {
    Method(Method),
    Signal(Signal)
}

pub struct Method {
    pub is_virtual: bool,
    pub item: ImplItem,
}

pub struct Signal {
    // FIXME: signal flags
    pub item: ImplItem
}
