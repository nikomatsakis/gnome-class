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
    Ok(quote! { #(#class_tokens)* })
}

struct ClassContext<'ast> {
    program: &'ast Program,
    class: &'ast Class,
    private_struct: &'ast PrivateStruct,
    GClassName: Identifier,
    MethodsDeclared: Identifier,
    ParentInstance: Tokens,
    ParentGClass: Tokens,
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

        let GClassName = Identifier {
            str: intern(&format!("{}Class", class.name.str))
        };

        // GObject is hardcoded in various places below
        assert!(class.extends.is_none());
        let ParentInstance = quote! { ::gobject_sys::GObject };
        let ParentGClass = quote! { ::gobject_sys::GObjectClass };

        let InstanceName = class.name;
        let MethodsDeclared = Identifier {
            str: intern(&format!("MethodsDeclaredIn{}", InstanceName.str))
        };

        Ok(ClassContext { program, class, private_struct, GClassName,
                          ParentInstance, ParentGClass, MethodsDeclared })
    }

    pub fn gen_class(&self) -> Result<Tokens> {
        let all = vec![
            self.type_decls(),
            self.impls(),
            self.methods_declared_in_instance(),
            self.always_impl(),
            self.method_redirects(),
            self.gclass_impl(),
        ];

        let mut result = Tokens::new();
        result.append_all(all);
        Ok(result)
    }

    fn type_decls(&self) -> Tokens {
        let InstanceName = self.class.name;
        let ParentInstance = &self.ParentInstance;
        let PrivateName = self.private_struct.name;
        let GClassName = self.GClassName;
        let ParentGClass = &&self.ParentGClass;

        let private_struct_fields = &self.private_struct.fields;

        let init_fn = self.init_fn();
        let method_names = &self.method_names();
        let method_fn_tys = &self.method_fn_tys();

        quote! {
            #[repr(C)]
            pub struct #InstanceName {
                parent: #ParentInstance,
                // FIXME: We need to add some way here to ensure that
            }

            struct #PrivateName {
                #(#private_struct_fields),*
            }

            impl #PrivateName {
                pub fn new() -> Self {
                    #init_fn
                }
            }

            #[repr(C)]
            pub struct #GClassName {
                parent_class: #ParentGClass,
                #(#method_names: Option<#method_fn_tys>,)*
            }
        }
    }

    fn impls(&self) -> Tokens {
        let InstanceName = self.class.name;
        let GClassName = self.GClassName;
        let ParentGClass = &self.ParentGClass;

        quote! {
            unsafe impl GInstance for #InstanceName {
                type Class = #GClassName;
            }

            unsafe impl GClass for #GClassName {
                type Instance = #InstanceName;
            }

            unsafe impl GSubclass for #GClassName {
                type ParentClass = #ParentGClass;
            }
        }
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

    fn method_assignments(&self) -> Vec<Tokens> {
        let InstanceName = self.class.name;
        let MethodsDeclared = &self.MethodsDeclared;
        self.method_names()
            .iter()
            .map(|method_name| {
                quote! { klass.#method_name = <#InstanceName as #MethodsDeclared>::#method_name; }
            })
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

    fn methods_declared_in_instance(&self) -> Tokens {
        let InstanceName = self.class.name;
        let method_trait_fns = &self.method_trait_fns();
        let method_impl_fns = &self.method_impl_fns();
        let MethodsDeclared = &self.MethodsDeclared;

        quote! {
            pub(super) trait Trait {
                #(#method_trait_fns)*
            }

            impl #MethodsDeclared for #InstanceName {
                #(#method_impl_fns)*
            }
        }
    }

    pub fn method_trait_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let args = &method.fn_def.sig.args;
                let return_ty = ReturnTy {
                    ty: method.fn_def.sig.return_ty.as_ref()
                };
                quote! {
                    fn #name(&self, #(#args),*) #return_ty;
                }
            })
            .collect()
    }

    pub fn method_impl_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let args = &method.fn_def.sig.args;
                let return_ty = ReturnTy {
                    ty: method.fn_def.sig.return_ty.as_ref()
                };
                let code = &method.fn_def.code;
                quote! {
                    fn #name(&self, #(#args),*) #return_ty {
                        #code
                    }
                }
            })
            .collect()
    }

    fn always_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let GClassName = self.GClassName;
        let PrivateName = self.private_struct.name;
        let ParentInstance = &self.ParentInstance;

        quote! {
            impl #InstanceName {
                pub fn new() -> G<#InstanceName> {
                    use gobject_sys::GObject;
                    use std::ptr;

                    unsafe {
                        let data: *mut GObject =
                            gobject_sys::g_object_new(#GClassName::class(),
                                                      ptr::null_mut());
                        G::new(data as *mut #InstanceName)
                    }
                }

                fn private(&self) -> &#PrivateName {
                    use gobject_sys::{self, GTypeInstance};

                    unsafe {
                        let this = self as *const #InstanceName as *mut GTypeInstance;
                        let private = gobject_sys::g_type_instance_get_private(this, #GClassName::class());
                        let private = private as *const #PrivateName;
                        &*private
                    }
                }

                pub fn to_ref(&self) -> G<#InstanceName> {
                    ::g::to_object_ref(self).clone()
                }

                pub fn upcast(&self) -> &#ParentInstance {
                    &self.parent
                }
            }
        }
    }

    fn method_redirects(&self) -> Tokens {
        let InstanceName = self.class.name;

        let method_tokens: Vec<_> =
            self.methods()
            .map(|method| {
                let name = method.name;
                let args = &method.fn_def.sig.args;
                let return_ty = ReturnTy {
                    ty: method.fn_def.sig.return_ty.as_ref()
                };
                quote! {
                    pub fn #name(&self, #(#args),*) #return_ty {
                        (::g::get_class(self).#name.unwrap())(
                            self, #(#args),*
                        )
                    }
                }
            })
            .collect();

        quote! {
            impl #InstanceName {
                #(#method_tokens)*
            }
        }
    }

    fn gclass_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let GClassName = self.GClassName;
        let ParentGClass = &self.ParentGClass;
        let PrivateName = self.private_struct.name;

        let method_assignments = self.method_assignments();

        quote! {
            impl #GClassName {
                pub fn gtype() -> GType {
                    use gobject_sys;
                    use std::mem;

                    extern fn instance_init(this: *mut GTypeInstance,
                                            _klass: gpointer)
                    {
                        unsafe {
                            extern crate gobject_sys;
                            use std::ptr;

                            let private = gobject_sys::g_type_instance_get_private(this, #GClassName::gtype());
                            let private = private as *mut #PrivateName;
                            ptr::write(private, #PrivateName::new());
                        }
                    }

                    extern fn class_init(klass: gpointer,
                                         _klass_data: gpointer)
                    {
                        extern "C" fn finalize(this: *mut gobject_sys::GObject) {
                            // TODO finalize fields
                        }

                        unsafe {
                            let g_object_class = klass as *mut GObjectClass;
                            (*g_object_class).finalize = Some(finalize);

                            gobject_sys::g_type_class_add_private(
                                klass,
                                mem::size_of::<#PrivateName>());

                            let klass = klass as *mut #GClassName;
                            let klass: &mut #GClassName = &mut *klass;
                            #(#method_assignments)*
                        }
                    }

                    fn register() -> GType {
                        unsafe {
                            gobject_sys::g_type_register_static_simple(
                                #ParentGClass::gtype(),
                                XXX, //b"Counter\0" as *const u8 as *const i8,
                                mem::size_of::<#GClassName>() as u32,
                                Some(class_init),
                                mem::size_of::<#InstanceName>() as u32,
                                Some(instance_init),
                                GTypeFlags::empty())
                        }
                    }

                    lazy_static! {
                        static ref GTYPE: GType = register();
                    }

                    *GTYPE
                }
            }
        }
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

        ReturnTy {
            ty: self.sig.return_ty.as_ref()
        }.to_tokens(tokens);
    }
}

struct ReturnTy<'ast> {
    ty: Option<&'ast Type>,
}

impl<'ast> ToTokens for ReturnTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if let Some(return_ty) = self.ty {
            tokens.append("->");
            return_ty.to_tokens(tokens);
        }
    }
}
