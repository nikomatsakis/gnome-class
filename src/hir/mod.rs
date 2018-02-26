// High-level Internal Representation of GObject artifacts
//
// Here we provide a view of the world in terms of what GObject knows:
// classes, interfaces, signals, etc.
//
// We construct this view of the world from the raw Abstract Syntax
// Tree (AST) from the previous stage.

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use syn::{self, Ident, Path, Block, ReturnType};
use syn::synom::Synom;
use syn::buffer::TokenBuffer;

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
    pub gobject_parent: bool,
    pub parent: Tokens,           // Parent
    pub parent_ffi: Tokens,       // ffi::Parent
    pub parent_class_ffi: Tokens, // ffi::ParentClass
    pub implements: Vec<Path>,    // names of GTypeInterfaces

    pub instance_private: Option<&'ast Path>,
    // pub class_private: Option<&'ast ast::PrivateStruct>

    // The order of these is important; it's the order of the slots in FooClass
    pub slots: Vec<Slot<'ast>>,
    // pub n_reserved_slots: usize,

    // pub properties: Vec<Property>,

    pub overrides: HashMap<Ident, Vec<Method<'ast>>>
}

pub enum Slot<'ast> {
    Method(Method<'ast>),
    VirtualMethod(VirtualMethod<'ast>),
    Signal(Signal)
}

pub struct Method<'ast> {
    pub public: bool,
    pub sig: FnSig<'ast>,
    pub body: &'ast Block,
}

pub struct VirtualMethod<'ast> {
    pub sig: FnSig<'ast>,
    pub body: Option<&'ast Block>,
}

pub struct FnSig<'ast> {
    pub name: Ident,
    pub inputs: Vec<FnArg<'ast>>,
    pub output: Ty<'ast>,
}

pub enum FnArg<'ast> {
    SelfRef(Token!(&), Token!(self)),
    Arg {
        mutbl: Option<Token![mut]>,
        name: Ident,
        ty: Ty<'ast>,
    }
}

pub struct Signal {
    // FIXME: signal flags
}

pub enum Ty<'ast> {
    Unit,
    Char(Ident),
    Bool(Ident),
    Borrowed(Box<Ty<'ast>>),
    Integer(Ident),
    Owned(&'ast syn::Path),
}

impl<'ast> Program<'ast> {
    pub fn from_ast_program(ast: &'ast ast::Program) -> Result<Program<'ast>> {
        check_program(ast)?;

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

    fn add(&mut self, ast_class: &'ast ast::Class) -> Result<()>
    {
        let prev = self.items.insert(ast_class.name, Class {
            name: ast_class.name,
            gobject_parent: ast_class.extends.is_none(),
            parent: tokens_ParentInstance(ast_class),
            parent_ffi: tokens_ParentInstanceFfi(ast_class),
            parent_class_ffi: tokens_ParentClassFfi(ast_class),
            implements: Vec::new(),
            instance_private: ast_class.items.iter().filter_map(|i| {
                match *i {
                    ast::ClassItem::InstancePrivate(ref ip) => Some(&ip.path),
                }
            }).next(),
            slots: Vec::new(),
            overrides: HashMap::new(),
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
        match impl_.trait_ {
            Some(parent_class) => {
                for item in impl_.items.iter() {
                    let item = match item.node {
                        ast::ImplItemKind::Method(ref m) => m,
                        ast::ImplItemKind::ReserveSlots(_) => {
                            bail!("can't reserve slots in a parent class impl");
                        }
                    };
                    if item.signal {
                        bail!("can't implement signals for parent classes")
                    }
                    if !item.virtual_ {
                        bail!("can only implement virtual functions for parent classes")
                    }
                    if item.public {
                        bail!("overrides are always public, no `pub` needed")
                    }
                    let method = match class.translate_method(item)? {
                        Slot::VirtualMethod(VirtualMethod { sig, body: Some(body) }) => {
                            Method { public: false, sig, body }
                        }
                        Slot::VirtualMethod(VirtualMethod { .. }) => {
                            bail!("overrides must provide a body for virtual \
                                   methods");
                        }
                        _ => unreachable!(),
                    };
                    class.overrides
                        .entry(parent_class)
                        .or_insert(Vec::new())
                        .push(method);
                }
            }
            None => {
                for item in impl_.items.iter() {
                    let slot = class.translate_slot(item)?;
                    class.slots.push(slot);
                }
            }
        }

        Ok(())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Class> + 'a {
        self.items.values()
    }
}

impl<'ast> Class<'ast> {
    fn translate_slot(&mut self, item: &'ast ast::ImplItem) -> Result<Slot<'ast>> {
        assert_eq!(item.attrs.len(), 0); // attributes unimplemented
        match item.node {
            ast::ImplItemKind::Method(ref method) => self.translate_method(method),
            ast::ImplItemKind::ReserveSlots(ref _slots) => {
                panic!("reserve slots not implemented");
            }
        }
    }

    fn translate_method(&mut self, method: &'ast ast::ImplItemMethod)
        -> Result<Slot<'ast>>
    {
        if method.signal {
            panic!("signals not implemented");
        }
        if method.virtual_ {
            if method.public {
                bail!("function `{}` is virtual so it doesn't need to be public",
                      method.name)
            }
            let sig = self.extract_sig(method)?;
            Ok(Slot::VirtualMethod(VirtualMethod {
                sig,
                body: method.body.as_ref(),
            }))
        } else {
            let sig = self.extract_sig(method)?;
            Ok(Slot::Method(Method {
                sig,
                public: method.public,
                body: method.body.as_ref().ok_or_else(|| {
                    format!("function `{}` requires a body", method.name)
                })?,
            }))
        }
    }

    fn extract_sig(&mut self, method: &'ast ast::ImplItemMethod) -> Result<FnSig<'ast>> {
        Ok(FnSig {
            output: self.extract_output(&method.output)?,
            inputs: self.extract_inputs(&method.inputs)?,
            name: method.name,
        })
    }

    fn extract_output(&mut self, output: &'ast ReturnType) -> Result<Ty<'ast>> {
        match *output {
            ReturnType::Type(_, ref boxt) => self.extract_ty(boxt),
            ReturnType::Default => Ok(Ty::Unit),
        }
    }

    fn extract_inputs(&mut self, t: &'ast [syn::FnArg]) -> Result<Vec<FnArg<'ast>>> {
        t.iter().map(|arg| {
            match *arg {
                syn::FnArg::Captured(syn::ArgCaptured { ref pat, ref ty, .. }) => {
                    let (name, mutbl) = match *pat {
                        syn::Pat::Ident(syn::PatIdent {
                            by_ref: None,
                            mutability: m,
                            ident,
                            subpat: None,
                        }) => {
                            (ident, m)
                        }
                        _ => bail!("only bare identifiers are allowed as \
                                    argument patterns"),
                    };

                    Ok(FnArg::Arg {
                        mutbl,
                        name,
                        ty: self.extract_ty(ty)?,
                    })
                }
                syn::FnArg::SelfRef(syn::ArgSelfRef {
                    and_token,
                    lifetime: None,
                    mutability: None,
                    self_token,
                }) => {
                    Ok(FnArg::SelfRef(and_token, self_token))
                }
                syn::FnArg::SelfRef(syn::ArgSelfRef {
                    mutability: Some(..),
                    ..
                }) => {
                    bail!("&mut self not implemented yet")
                }
                syn::FnArg::SelfRef(syn::ArgSelfRef {
                    lifetime: Some(..),
                    ..
                }) => {
                    bail!("lifetime arguments on self not implemented yet")
                }
                syn::FnArg::SelfValue(_) => bail!("by-value self not implemented"),
                syn::FnArg::Ignored(_) => bail!("cannot have ignored function arguments"),
            }
        }).collect()
    }

    fn extract_ty(&mut self, t: &'ast syn::Type) -> Result<Ty<'ast>> {
        match *t {
            syn::Type::Slice(_) => bail!("slice types not implemented yet"),
            syn::Type::Array(_) => bail!("array types not implemented yet"),
            syn::Type::Ptr(_) => bail!("ptr types not implemented yet"),
            syn::Type::Reference(syn::TypeReference { lifetime: Some(_), .. }) => {
                bail!("borrowed types with lifetimes not implemented yet")
            }
            syn::Type::Reference(syn::TypeReference { lifetime: None, ref elem, ref mutability, .. }) => {
                if let Some(_) = *mutability {
                    bail!("mutable borrowed pointers not implemented");
                }
                let path = match **elem {
                    syn::Type::Path(syn::TypePath { qself: None, ref path }) => path,
                    _ => bail!("only borrowed pointers to paths supported"),
                };
                let ty = self.extract_ty_path(path)?;
                Ok(Ty::Borrowed(Box::new(ty)))
            }
            syn::Type::BareFn(_) => bail!("function pointer types not implemented yet"),
            syn::Type::Never(_) => bail!("never not implemented yet"),
            syn::Type::Tuple(syn::TypeTuple { ref elems, .. }) => {
                if elems.len() == 0 {
                    Ok(Ty::Unit)
                } else {
                    bail!("tuple types not implemented yet")
                }
            }
            syn::Type::Path(syn::TypePath { qself: Some(_), .. }) => {
                bail!("path types with qualified self (`as` syntax) not allowed")
            }
            syn::Type::Path(syn::TypePath { qself: None, ref path }) => {
                self.extract_ty_path(path)
            }
            syn::Type::TraitObject(_) => bail!("trait objects not implemented yet"),
            syn::Type::ImplTrait(_) => bail!("trait objects not implemented yet"),
            syn::Type::Paren(syn::TypeParen { ref elem, .. }) => self.extract_ty(elem),
            syn::Type::Group(syn::TypeGroup { ref elem, .. }) => self.extract_ty(elem),
            syn::Type::Infer(_) => bail!("underscore types not allowed"),
            syn::Type::Macro(_) => bail!("type macros not allowed"),
        }
    }

    fn extract_ty_path(&mut self, t: &'ast syn::Path) -> Result<Ty<'ast>> {
        if t.segments.iter().any(|segment| {
            match segment.arguments {
                syn::PathArguments::None => false,
                _ => true,
            }
        }) {
            bail!("type or lifetime parameters not allowed")
        }
        if t.leading_colon.is_some() || t.segments.len() > 1 {
            return Ok(Ty::Owned(t))
        }

        let ident = t.segments.get(0).item().ident;

        match ident.as_ref() {
            "char" => Ok(Ty::Char(ident)),
            "bool" => Ok(Ty::Bool(ident)),
            "i8" |
            "i16" |
            "i32" |
            "i64" |
            "isize" |
            "u8" |
            "u16" |
            "u32" |
            "u64" |
            "usize" => {
                Ok(Ty::Integer(ident))
            }
            _other => Ok(Ty::Owned(t)),
        }
    }
}

fn make_path_glib_object() -> Path {
    let tokens = quote_cs! { glib::Object };
    let token_stream = TokenStream::from(tokens);
    let buffer = TokenBuffer::new(token_stream);
    let cursor = buffer.begin();
    Path::parse(cursor).unwrap().0
}

impl<'a> ToTokens for FnArg<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            FnArg::SelfRef(and, self_) => {
                and.to_tokens(tokens);
                self_.to_tokens(tokens);
            }
            FnArg::Arg { name, ref ty, mutbl } => {
                mutbl.to_tokens(tokens);
                name.to_tokens(tokens);
                Default::<Token!(:)>::default().to_tokens(tokens);
                ty.to_tokens(tokens);
            }
        }
    }
}

impl<'a> ToTokens for Ty<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Ty::Unit => tokens.append_delimited("(", Default::default(), |_| ()),
            Ty::Char(tok) => tok.to_tokens(tokens),
            Ty::Bool(tok) => tok.to_tokens(tokens),
            Ty::Integer(t) => t.to_tokens(tokens),
            Ty::Borrowed(ref t) => {
                Default::<Token!(&)>::default().to_tokens(tokens);
                t.to_tokens(tokens)
            }
            Ty::Owned(t) => t.to_tokens(tokens),
        }
    }
}

pub mod tests {
    use super::*;

    pub fn run() {
        creates_trivial_class();
        creates_class_with_superclass();
    }

    fn test_class_and_superclass (raw: &str, class_name: &str, superclass_name: &str) {
        let token_stream = raw.parse::<TokenStream>().unwrap();
        let buffer = TokenBuffer::new(token_stream);
        let cursor = buffer.begin();
        let ast_program = ast::Program::parse(cursor).unwrap().1;

        let program = Program::from_ast_program(&ast_program).unwrap();

        assert!(program.classes.len() == 1);

        let class = program.classes.get(class_name);
        assert_eq!(class.name.as_ref(), class_name);
        assert_eq!(class.parent.to_string(), superclass_name);
    }

    fn creates_trivial_class() {
        let raw = "class Foo {}";

        test_class_and_superclass(raw, "Foo", "glib :: Object");
    }

    fn creates_class_with_superclass() {
        let raw = "class Foo: Bar {}";

        test_class_and_superclass(raw, "Foo", "Bar");
    }
}
