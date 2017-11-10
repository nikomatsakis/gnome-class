// High-level Internal Representation of GObject artifacts
//
// Here we provide a view of the world in terms of what GObject knows:
// classes, interfaces, signals, etc.
//
// We construct this view of the world from the raw Abstract Syntax
// Tree (AST) from the previous stage.

use std::collections::HashMap;

use proc_macro2::{TokenStream};
use quote::{Tokens};
use syn::{Ident, Path, Block, FnArg, FunctionRetTy};
use synom::{Synom, SynomBuffer};

use super::ast;
use super::checking::*;
use super::errors::*;
use super::glib_utils::*;

pub struct Program<'ast> {
    pub classes: Classes<'ast>,
}

pub struct Classes<'ast> {
    items: HashMap<Ident, Class<'ast>>,
}

pub struct Class<'ast> {
    pub name: Ident,              // Foo
    pub parent: Tokens,           // Parent
    pub parent_ffi: Tokens,       // ffi::Parent
    pub parent_class_ffi: Tokens, // ffi::ParentClass
    pub implements: Vec<Path>,    // names of GTypeInterfaces

    pub instance_private: Option<&'ast ast::PrivateStruct>,
    // pub class_private: Option<&'ast ast::PrivateStruct>

    // The order of these is important; it's the order of the slots in FooClass
    pub slots: Vec<Slot<'ast>>,
    // pub n_reserved_slots: usize,

    // pub properties: Vec<Property>,
}

pub enum Slot<'ast> {
    Method(Method<'ast>),
    VirtualMethod(VirtualMethod<'ast>),
    Signal(Signal)
}

pub struct Method<'ast> {
    pub public: bool,
    pub name: Ident,
    pub inputs: &'ast [FnArg],
    pub output: &'ast FunctionRetTy,
    pub body: &'ast Block,
}

pub struct VirtualMethod<'ast> {
    pub name: Ident,
    pub inputs: &'ast [FnArg],
    pub output: &'ast FunctionRetTy,
    pub body: Option<&'ast Block>,
}

pub struct Signal {
    // FIXME: signal flags
}

fn get_instance_private<'ast>(ast_program: &'ast ast::Program) -> Option<&'ast ast::PrivateStruct> {
    ast_program.items.iter().filter_map(|i| {
        if let ast::Item::PrivateStruct(ref p) = *i {
            Some(p)
        } else {
            None
        }
    }).next()
}

impl<'ast> Program<'ast> {
    pub fn from_ast_program(ast: &'ast ast::Program) -> Result<Program<'ast>> {
        check_program(ast)?;

        let mut classes = Classes::new();
        for class in ast.classes() {
            classes.add(class, get_instance_private(ast))?;
        }
        for impl_ in ast.impls() {
            classes.add_impl(impl_)?;
        }

        Ok(Program {
            classes: classes,
        })
    }
}

impl<'ast> Classes<'ast> {
    fn new() -> Classes<'ast> {
        Classes {
            items: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, name: &str) -> &Class {
        self.items.iter().find(|c| c.1.name == name).unwrap().1
    }

    fn add(&mut self,
           ast_class: &'ast ast::Class,
           instance_private: Option<&'ast ast::PrivateStruct>) -> Result<()>
    {
        let prev = self.items.insert(ast_class.name, Class {
            name: ast_class.name,
            parent: tokens_ParentInstance(ast_class),
            parent_ffi: tokens_ParentInstanceFfi(ast_class),
            parent_class_ffi: tokens_ParentClassFfi(ast_class),
            implements: Vec::new(),
            instance_private,
            slots: Vec::new()
        });
        if prev.is_some() {
            bail!("redefinition of class `{}`", ast_class.name);
        }
        Ok(())
    }

    fn add_impl(&mut self, impl_: &'ast ast::Impl) -> Result<()> {
        let class = match self.items.get_mut(&impl_.self_path) {
            Some(class) => class,
            None => bail!("impl for class that doesn't exist: {}", impl_.self_path),
        };
        if impl_.trait_.is_some() {
            // would want to attach destructors/such here
            panic!("trait-like impls not implemented");
        } else {
            for item in impl_.items.iter() {
                class.add_slot(item)?;
            }
        }

        Ok(())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Class> + 'a {
        self.items.values()
    }
}

impl<'ast> Class<'ast> {
    fn add_slot(&mut self, item: &'ast ast::ImplItem) -> Result<()> {
        assert_eq!(item.attrs.len(), 0); // attributes unimplemented
        match item.node {
            ast::ImplItemKind::Method(ref method) => self.add_method(method),
            ast::ImplItemKind::ReserveSlots(ref _slots) => {
                panic!("reserve slots not implemented");
            }
        }
    }

    fn add_method(&mut self, method: &'ast ast::ImplItemMethod) -> Result<()> {
        if method.signal {
            panic!("signals not implemented");
        }
        if method.virtual_ {
            if method.public {
                bail!("function `{}` is virtual so it doesn't need to be public",
                      method.name)
            }
            self.slots.push(Slot::VirtualMethod(VirtualMethod {
                name: method.name,
                inputs: &method.inputs,
                output: &method.output,
                body: method.body.as_ref(),
            }));
        } else {
            self.slots.push(Slot::Method(Method {
                name: method.name,
                inputs: &method.inputs,
                output: &method.output,
                public: method.public,
                body: method.body.as_ref().ok_or_else(|| {
                    format!("function `{}` requires a body", method.name)
                })?,
            }));
        }
        Ok(())
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

    fn test_class_and_superclass (raw: &str, class_name: &str, superclass_name: &str) {
        let token_stream = raw.parse::<TokenStream>().unwrap();
        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();
        let ast_program = ast::Program::parse(cursor).unwrap().1;

        let program = Program::from_ast_program(&ast_program).unwrap();

        assert!(program.classes.len() == 1);

        let class = program.classes.get(class_name);
        assert_eq!(class.name.as_ref(), class_name);
        assert_eq!(class.parent.to_string(), superclass_name);
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
