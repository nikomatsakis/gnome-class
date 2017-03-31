// We give `ClassName` variables an identifier that uses upper-case.
#![allow(non_snake_case)]

use ast::*;
use errors::*;
use lalrpop_intern::{self, intern};
use quote::{Ident, Tokens, ToTokens};
use std::convert::Into;

pub fn classes(program: &Program) -> Result<Tokens> {
    let class_tokens =
        program.classes
               .iter()
               .map(|class| {
                   let cx = ClassContext::new(program, class)?;
                   cx.gen_class()
               })
               .collect::<Result<Vec<_>>>()?;
    println!("{:?}", class_tokens);
    Ok(quote! { #(#class_tokens)* })
}

struct ClassContext<'ast> {
    program: &'ast Program,
    class: &'ast Class,
    private_struct: &'ast PrivateStruct,
    gclass_name: Identifier,
}

impl<'ast> ClassContext<'ast> {
    pub fn new(program: &'ast Program, class: &'ast Class) -> Result<Self> {
        let private_struct =
            class.members
                 .iter()
                 .filter_map(|member| match *member {
                     Member::PrivateStruct(ref ps) => Some(ps),
                     _ => None,
                 })
                 .next();

        let private_struct = match private_struct {
            Some(p) => p,
            None => bail!("no private struct found")
        };

        let gclass_name = Identifier {
            str: intern(&format!("{}Class", class.name.str))
        };

        Ok(ClassContext { program, class, private_struct, gclass_name })
    }

    pub fn gen_class(&self) -> Result<Tokens> {
        let InstanceName = self.class.name;
        let PrivateName = self.private_struct.name;
        let GClassName = self.gclass_name;

        let private_struct_fields = &self.private_struct.fields;
        let init_fn = self.init_fn();
        let method_names = self.method_names();
        let method_fn_tys = self.method_fn_tys();

        // GObject is hardcoded in various places below
        assert!(self.class.extends.is_none());
        let ParentInstance = quote! { ::gobject_sys::GObject };
        let ParentGClass = quote! { ::gobject_sys::GObjectClass };

        Ok(quote! {
            #[repr(C)]
            pub struct #InstanceName {
                parent: #ParentInstance,
                // FIXME: We need to add some way here to ensure that
            }

            struct #PrivateName {
                #(#private_struct_fields),*
            }

            pub struct #GClassName {
                parent_class: #ParentGClass,
                #(#method_names: Option<#method_fn_tys>,)*
            }

            impl #InstanceName {
                extern fn init(this: *mut GTypeInstance,
                               _klass: gpointer)
                {
                    fn new() -> #PrivateName {
                        #init_fn
                    }

                    unsafe {
                        extern crate gobject_sys;
                        use std::ptr;

                        let private = gobject_sys::g_type_instance_get_private(this, #GClassName::gtype());
                        let private = private as *mut #PrivateName;
                        ptr::write(private, new());
                    }
                }
            }
        })
    }

    pub fn init_fn(&self) -> Tokens {
        let init_member = self.class.members
                                    .iter()
                                    .filter_map(|m| match *m {
                                        Member::Init(ref f) => Some(f),
                                        _ => None,
                                    })
                                    .next();
        if let Some(i) = init_member {
            quote! { #i }
        } else {
            let PrivateName = self.private_struct.name;
            quote! { #PrivateName::default() }
        }
    }

    pub fn methods(&self) -> impl Iterator<Item = &'ast Method> {
        self.class
            .members
            .iter()
            .filter_map(|member| match *member {
                Member::Method(ref m) => Some(m),
                _ => None,
            })
    }

    pub fn method_names(&self) -> Vec<Identifier> {
        self.methods()
            .map(|method| method.name)
            .collect()
    }

    pub fn method_fn_tys(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let method_fn_ty = MethodTy {
                    class_name: self.class.name,
                    sig: &method.fn_def.sig
                };
                quote! { #method_fn_ty }
            })
            .collect()
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.name.to_tokens(tokens);
        tokens.append(":");
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Type::Name(id) => id.to_tokens(tokens),
            Type::Args(id, ref tys) => {
                let q = quote!{ #id < #(#tys),* > };
                tokens.append_all(Some(q));
            }
            Type::Array(ref ty) => {
                let q = quote!{ [ #ty ] };
                tokens.append_all(Some(q));
            }
            Type::Sum(ref tys) => {
                let q = quote!{ #(#tys)+* };
                tokens.append_all(Some(q));
            }
        }
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut Tokens) {
        lalrpop_intern::read(|interner| {
            Ident::new(interner.data(self.str)).to_tokens(tokens);
        })
    }
}

impl ToTokens for OpaqueTokens {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.tokens.to_tokens(tokens)
    }
}

struct MethodTy<'ast> {
    class_name: Identifier,
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for MethodTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        tokens.append("extern fn(");

        tokens.append("&");
        self.class_name.to_tokens(tokens);
        tokens.append(", ");

        for arg in &self.sig.args {
            arg.ty.to_tokens(tokens);
            tokens.append(", ");
        }
        tokens.append(")");
        if let Some(ref return_ty) = self.sig.return_ty {
            tokens.append("->");
            return_ty.to_tokens(tokens);
        }
    }
}

