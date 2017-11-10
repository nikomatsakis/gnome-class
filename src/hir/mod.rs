// High-level Internal Representation of GObject artifacts
//
// Here we provide a view of the world in terms of what GObject knows:
// classes, interfaces, signals, etc.
//
// We construct this view of the world from the raw Abstract Syntax
// Tree (AST) from the previous stage.

use std::collections::HashMap;

use proc_macro2::{TokenStream};
use syn::{Ident, Path, Block, FnArg, FunctionRetTy};
use synom::{Synom, SynomBuffer};

use super::ast;
use super::checking::*;
use super::errors::*;

pub struct Program<'ast> {
    pub classes: Classes<'ast>,
}

pub struct Classes<'ast> {
    items: HashMap<Ident, Class<'ast>>,
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
    VirtualMethod(VirtualMethod),
    Signal(Signal)
}

pub struct Method {
    pub public: bool,
    pub name: Ident,
    pub inputs: Vec<FnArg>,
    pub output: FunctionRetTy,
    pub body: Block,
}

pub struct VirtualMethod {
    pub name: Ident,
    pub inputs: Vec<FnArg>,
    pub output: FunctionRetTy,
    pub body: Option<Block>,
}

pub struct Signal {
    // FIXME: signal flags
}

impl<'ast> Program<'ast> {
    pub fn from_ast_program(ast_program: ast::Program) -> Result<Program<'ast>> {
        let ast = check_program(ast_program)?;

        let mut classes = Classes::new();
        for class in ast.classes() {
            classes.add(class)?;
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

    fn add(&mut self, class: &ast::Class) -> Result<()> {
        let prev = self.items.insert(class.name, Class {
            name: class.name,
            superclass: class.extends.clone().unwrap_or(make_path_glib_object()),
            implements: Vec::new(),
            instance_private: None,
            slots: Vec::new()
        });
        if prev.is_some() {
            bail!("redefinition of class `{}`", class.name);
        }
        Ok(())
    }

    fn add_impl(&mut self, impl_: &ast::Impl) -> Result<()> {
        let class = match self.items.get_mut(&impl_.self_path) {
            Some(class) => class,
            None => bail!("impl for class that doesn't exist: {}", impl_.self_path),
        };
        if impl_.trait_.is_some() {
            // would want to attach destructors/such here
            unimplemented!()
        } else {
            for item in impl_.items.iter() {
                class.add_slot(item)?;
            }
        }

        Ok(())
    }
}

impl<'ast> Class<'ast> {
    fn add_slot(&mut self, item: &ast::ImplItem) -> Result<()> {
        assert_eq!(item.attrs.len(), 0); // attributes unimplemented
        match item.node {
            ast::ImplItemKind::Method(ref method) => self.add_method(method),
            ast::ImplItemKind::ReserveSlots(ref _slots) => {
                unimplemented!()
            }
        }
    }

    fn add_method(&mut self, method: &ast::ImplItemMethod) -> Result<()> {
        if method.signal {
            unimplemented!()
        }
        if method.virtual_ {
            if method.public {
                bail!("function `{}` is virtual so it doesn't need to be public",
                      method.name)
            }
            self.slots.push(Slot::VirtualMethod(VirtualMethod {
                name: method.name,
                inputs: method.inputs.clone(),
                output: method.output.clone(),
                body: method.body.clone(),
            }));
        } else {
            self.slots.push(Slot::Method(Method {
                name: method.name,
                inputs: method.inputs.clone(),
                output: method.output.clone(),
                public: method.public,
                body: method.body.clone().ok_or_else(|| {
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
    use quote::{ToTokens};

    fn test_class_and_superclass (raw: &str, class_name: &str, superclass_name: &str) {
        let token_stream = raw.parse::<TokenStream>().unwrap();
        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();
        let ast_program = ast::Program::parse(cursor).unwrap().1;

        let program = Program::from_ast_program(ast_program).unwrap();

        assert!(program.classes.len() == 1);

        let class = program.classes.get(class_name);
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
