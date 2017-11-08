// High-level Internal Representation of GObject artifacts
//
// Here we provide a view of the world in terms of what GObject knows:
// classes, interfaces, signals, etc.
//
// We construct this view of the world from the raw Abstract Syntax
// Tree (AST) from the previous stage.

use proc_macro2::{TokenStream};
use syn::{Ident, ImplItem, Path};
use synom::{Synom, SynomBuffer};

use super::ast;
use super::ast::get_program_classes;
use super::checking::*;
use super::errors::*;

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

impl<'ast> Program<'ast> {
    pub fn from_ast_program(ast_program: ast::Program) -> Result<Program<'ast>> {
        let ast_program = check_program(ast_program)?;

        Ok(Program {
            classes: Self::extract_classes (ast_program)
        })
    }

    fn extract_classes(ast_program: ast::Program) -> Vec<Class<'ast>> {
        let mut classes = Vec::new();

        for ast_class in get_program_classes(&ast_program) {
            classes.push (Self::extract_class (ast_class));
        }

        classes
    }

    fn extract_class(ast_class: &ast::Class) -> Class<'ast> {
        Class {
            name: ast_class.name.clone(),
            superclass: ast_class.extends.clone().unwrap_or(make_path_glib_object()),
            implements: Vec::new(),
            instance_private: None,
            slots: Vec::new()
        }
    }
}

fn make_path_glib_object() -> Path {
    let tokens = quote! { glib::Object };
    let token_stream = TokenStream::from(tokens);
    let buffer = SynomBuffer::new(token_stream);
    let cursor = buffer.begin();
    Path::parse(cursor).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{ToTokens};

    fn test_class_and_superclass (raw: &str, class_name: &str, superclass_name: &str) {
        let token_stream = raw.parse::<TokenStream>().unwrap();
        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();
        let ast_program = ast::Program::parse(cursor).unwrap().1;

        let program = Program::from_ast_program(ast_program).unwrap();

        assert!(program.classes.len() == 1);

        let class = &program.classes[0];
        assert_eq!(class.name.as_ref(), class_name);
        assert_eq!(class.superclass.clone().into_tokens().to_string(), superclass_name);
    }

    #[test]
    fn creates_trivial_class() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                           }
                       }
                   }";

        test_class_and_superclass(raw, "Foo", "glib :: Object");
    }

    #[test]
    fn creates_class_with_superclass() {
        let raw = "class Foo: Bar {
                       struct FooPrivate {
                           foo: u32,
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                           }
                       }
                   }";

        test_class_and_superclass(raw, "Foo", "Bar");
    }
}
