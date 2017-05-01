// We give `ClassName` variables an identifier that uses upper-case.
#![allow(non_snake_case)]

use ast::*;
use errors::*;
use lalrpop_intern::{self, intern};
use quote::{Ident, Tokens, ToTokens};
use std::convert::Into;

macro_rules! quote_in {
    ($tokens:expr, $($t:tt)*) => {
        $tokens.append_all(Some(quote!{$($t)*}));
    }
}

// HYGIENE NOTE:
//
// I am using the `__` prefix to indicate names that, while visible
// to the user, are eventually intended to be hidden by hygiene.

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
    MethodsFrom: Identifier,
    ParentInstance: Tokens,
    ParentGClass: Tokens,
    GObject: Tokens,
    GObjectClass: Tokens,
    InstanceTrait: Identifier,
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

        let GObject = quote! { ::gnome_class_shims::gobject_sys::GObject };
        let GObjectClass = quote! { ::gnome_class_shims::gobject_sys::GObjectClass };

        // GObject is hardcoded in various places below
        let ParentInstance =
            class.extends
                 .as_ref()
                 .map(|c| c.ty())
                 .map(|c| quote! { #c })
                 .unwrap_or_else(|| GObject.clone());
        let ParentGClass = quote! {
            <#ParentInstance as ::gnome_class_shims::GInstance>::Class
        };

        let InstanceName = class.name;
        let MethodsFrom = Identifier {
            str: intern(&format!("__methods_from_{}", InstanceName.str))
        };

        let InstanceTrait = Identifier {
            str: intern(&format!("{}Trait", class.name.str))
        };

        Ok(ClassContext {
            program,
            class,
            private_struct,
            GClassName,
            ParentInstance,
            ParentGClass,
            MethodsFrom,
            GObject,
            GObjectClass,
            InstanceTrait,
        })
    }

    pub fn gen_class(&self) -> Result<Tokens> {
        let all = vec![
            self.type_decls(),
            self.impls(),
            self.methods_declared_in_instance(),
            self.always_impl(),
            self.instance_trait(),
            self.instance_trait_impl(),
            self.c_symbols(),
        ];

        Ok(quote! { #(#all)* })
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

                // The GObject runtime will store some private data in here.
                _phantom: ::std::marker::PhantomData<#PrivateName>,

                // This type, while zero-sized, is not constructible.
                // This assures that no Rust code can ever create an
                // instance of `#InstanceName` -- the only instances
                // are created by the GObject runtime. This is
                // important for the `GInstance` impl.
                _no_construct: ::gnome_class_shims::NoConstruct,
            }

            struct #PrivateName {
                #(#private_struct_fields),*
            }

            impl #PrivateName {
                pub fn new() -> Self #init_fn
            }

            #[repr(C)]
            pub struct #GClassName {
                parent_class: #ParentGClass,
                #(#method_names: Option<#method_fn_tys>,)*
            }
        }
    }

    fn instance_trait(&self) -> Tokens {
        let InstanceTrait = self.InstanceTrait;
        let method_trait_fns = &self.method_trait_fns();

        quote! {
            pub trait #InstanceTrait {
                #(#method_trait_fns)*
            }
        }
    }

    fn instance_trait_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let InstanceTrait = self.InstanceTrait;
        let method_redirects = self.method_redirects();
        quote! {
            impl #InstanceTrait for #InstanceName {
                #(#method_redirects)*
            }
        }
    }

    fn impls(&self) -> Tokens {
        let InstanceName = self.class.name;
        let GClassName = self.GClassName;
        let ParentGClass = &self.ParentGClass;

        let get_type_fn = self.get_type_fn();

        quote! {
            unsafe impl ::gnome_class_shims::GInstance for #InstanceName {
                type Class = #GClassName;

                #get_type_fn
            }

            unsafe impl ::gnome_class_shims::GClass for #GClassName {
                type Instance = #InstanceName;
            }

            unsafe impl ::gnome_class_shims::GSubclass for #GClassName {
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
            let private_struct_field_names =
                self.private_struct.fields
                                   .iter()
                                   .map(|f| f.name);
            quote! {
                {
                    #PrivateName {
                        #(#private_struct_field_names: Default::default()),*
                    }
                }
            }
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
        let MethodsFrom = &self.MethodsFrom;
        self.method_names()
            .iter()
            .map(|method_name| {
                quote! { klass.#method_name = Some(<#InstanceName as #MethodsFrom::Trait>::#method_name); }
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
        let MethodsFrom = &self.MethodsFrom;

        quote! {
            #[allow(non_snake_case)]
            mod #MethodsFrom {
                use super::*;
                pub trait Trait {
                    #(extern #method_trait_fns)*
                }
            }

            impl #MethodsFrom::Trait for #InstanceName {
                #(extern #method_impl_fns)*
            }
        }
    }

    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32);
    /// ```
    pub fn method_trait_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty;
                }
            })
            .collect()
    }

    /// Returns, for each method, something like
    ///
    /// ```notest
    /// fn foo(&self, arg: u32) { ... }
    /// ```
    pub fn method_impl_fns(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let return_ty = method.fn_def.sig.return_ty();
                let code = &method.fn_def.code;
                quote! {
                    fn #name(&self, #arg_decls) #return_ty #code
                }
            })
            .collect()
    }

    fn always_impl(&self) -> Tokens {
        let InstanceName = self.class.name;
        let PrivateName = self.private_struct.name;
        let ParentInstance = &self.ParentInstance;

        quote! {
            impl #InstanceName {
                pub fn new() -> ::gnome_class_shims::G<#InstanceName> {
                    use gnome_class_shims::G;
                    use gnome_class_shims::GInstance;
                    use gnome_class_shims::gobject_sys::{self, GObject};
                    use std::ptr;

                    unsafe {
                        let data: *mut GObject =
                            gobject_sys::g_object_new(
                                #InstanceName::get_type(),
                                ptr::null_mut());
                        G::new(data as *mut #InstanceName)
                    }
                }

                #[allow(dead_code)]
                fn private(&self) -> &#PrivateName {
                    use gnome_class_shims::GInstance;
                    use gnome_class_shims::gobject_sys::{self, GTypeInstance};

                    unsafe {
                        let this = self as *const #InstanceName as *mut GTypeInstance;
                        let private = gobject_sys::g_type_instance_get_private(this, #InstanceName::get_type());
                        let private = private as *const #PrivateName;
                        &*private
                    }
                }

                #[allow(dead_code)]
                pub fn to_ref(&self) -> ::gnome_class_shims::G<#InstanceName> {
                    ::gnome_class_shims::to_object_ref(self).clone()
                }

                #[allow(dead_code)]
                pub fn upcast(&self) -> &#ParentInstance {
                    &self.parent
                }
            }
        }
    }

    /// Returns, for each method `foo`, something like:
    ///
    /// ```notest
    /// fn foo(&self, arg: u32) {
    ///     (get_class(self).foo.unwrap())(self, arg)
    /// }
    /// ```
    fn method_redirects(&self) -> Vec<Tokens> {
        self.methods()
            .map(|method| {
                let name = method.name;
                let arg_decls = method.fn_def.sig.arg_decls();
                let arg_names = method.fn_def.sig.arg_names();
                let return_ty = method.fn_def.sig.return_ty();
                quote! {
                    fn #name(&self, #arg_decls) #return_ty {
                        let klass = ::gnome_class_shims::get_class(self);
                        (klass.#name.unwrap())(self, #arg_names)
                    }
                }
            })
            .collect()
    }

    fn lower_case_class_name(&self) -> String {
        lalrpop_intern::read(|interner| {
            let name_str = interner.data(self.class.name.str);
            let mut name_chars = name_str.chars();
            let first_char: char = name_chars.next().unwrap();
            first_char.to_lowercase().chain(name_chars).collect()
        })
    }

    fn c_symbols(&self) -> Tokens {
        let InstanceName = self.class.name;
        let instanceName = self.lower_case_class_name();

        let method_tokens: Vec<_> =
            self.methods()
                .map(|method| {
                    let arg_decls = method.fn_def.sig.arg_decls();
                    let arg_names = method.fn_def.sig.arg_names();
                    let return_ty = method.fn_def.sig.return_ty();
                    let name = method.name;
                    let c_name = Ident::new(format!("{}_{}",
                                                    instanceName,
                                                    method.name.str));
                    quote! {
                        #[no_mangle]
                        pub extern fn #c_name(__this: &#InstanceName, #arg_decls) #return_ty {
                            #InstanceName::#name(__this, #arg_names)
                        }
                    }
                })
                .collect();

        let get_type_name = Ident::new(format!("{}_get_type",
                                               instanceName));
        quote! {
            #[no_mangle]
            pub extern fn #get_type_name() -> ::gnome_class_shims::glib_sys::GType
            {
                use gnome_class_shims::GInstance;
                #InstanceName::get_type()
            }

            #(#method_tokens)*
        }
    }

    fn get_type_fn(&self) -> Tokens {
        let InstanceName = self.class.name;
        let GClassName = self.GClassName;
        let ParentInstance = &self.ParentInstance;
        let PrivateName = self.private_struct.name;

        // The function which initializes an instance of our class.
        // It simply sets up the private fields.
        let instance_init = quote! {
            extern fn instance_init(this: *mut GTypeInstance,
                                    _klass: gpointer)
            {
                unsafe {
                    let private = gobject_sys::g_type_instance_get_private(this, #InstanceName::get_type());
                    let private = private as *mut #PrivateName;
                    ptr::write(private, #PrivateName::new());
                }
            }
        };

        // The finalizer. It drops the private fields and then invokes
        // the parent class finalizer (which it loads from the parent
        // class struct).
        let finalize = quote! {
            extern fn finalize(this: *mut GObject) {
                let this = this as *mut #InstanceName;
                unsafe {
                    ptr::read((*this).private());

                    let object_class = shims::get_class(&*this);
                    let parent_class = shims::get_parent_class(object_class);
                    if let Some(f) = parent_class.finalize {
                        f(this as *mut GObject);
                    }
                }
            }
        };

        // Class initializer. Sets up the finalizer, private field
        // size, and initializes the fields for each of our methods.
        let method_assignments = self.method_assignments();
        let class_init = quote! {
            extern fn class_init(klass: gpointer,
                                 _klass_data: gpointer)
            {
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
        };

        // Registration function. Creates the GType. Intended to be run
        // at most once, returns the `GType` we created.
        let byte_string = ByteString(self.class.name);
        let register = quote! {
            fn register() -> GType {
                unsafe {
                    gobject_sys::g_type_register_static_simple(
                        #ParentInstance::get_type(),
                        #byte_string as *const u8 as *const i8,
                        mem::size_of::<#GClassName>() as u32,
                        Some(class_init),
                        mem::size_of::<#InstanceName>() as u32,
                        Some(instance_init),
                        GTypeFlags::empty())
                }
            }
        };

        quote! {
            fn get_type() -> ::gnome_class_shims::glib_sys::GType {
                use gnome_class_shims as shims;
                use gnome_class_shims::gobject_sys::{self,
                                                     GObject,
                                                     GObjectClass,
                                                     GTypeInstance,
                                                     GTypeFlags};
                use gnome_class_shims::glib_sys::{GType, gpointer};
                use std::{mem, ptr};

                // All these helper functions are intentionally
                // hidden inside of `get_type` so as not to
                // pollute the user's namespace.
                #instance_init
                #finalize
                #class_init
                #register

                lazy_static! {
                    static ref GTYPE: GType = register();
                }

                *GTYPE
            }
        }
    }
}

impl ToTokens for VarTy {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &VarTy { name, ref ty } = self;
        quote_in!(tokens, #name: #ty)
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Type::Path(ref path) => {
                path.ty().to_tokens(tokens)
            }
            Type::Array(ref ty) => {
                quote_in!(tokens, [ #ty ]);
            }
            Type::Sum(ref tys) => {
                quote_in!(tokens, #(#tys)+*);
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

struct ByteString(Identifier);

impl ToTokens for ByteString {
    fn to_tokens(&self, tokens: &mut Tokens) {
        lalrpop_intern::read(|interner| {
            // Because we are converting a legal identifier, we don't
            // have to worry about it having escape characters in it
            // or anything else:
            let mut s = String::new();
            s.push_str("b\"");
            s.push_str(interner.data(self.0.str));
            s.push_str("\\0\"");
            tokens.append(&s);
        })
    }
}

impl ToTokens for CodeBlock {
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
        let class_name = self.class_name;
        let arg_tys = self.sig.args.iter().map(|a| &a.ty);
        let return_ty = self.sig.return_ty();

        quote_in! {
            tokens,
            extern fn(&#class_name, #(#arg_tys),*) #return_ty
        }
    }
}

/// Helper methods for printing out various bits of
/// a method signature. For each of the comments below,
/// assume an example method `fn get(&self, x: u32, y: u32) -> u32`.
impl FnSig {
    /// Generates `x: u32, y: u32`
    fn arg_decls<'ast>(&'ast self) -> ArgDecls<'ast> {
        ArgDecls { sig: self }
    }

    /// Generates `x, y`
    fn arg_names<'ast>(&'ast self) -> ArgNames<'ast> {
        ArgNames { sig: self }
    }

    /// Generates `-> u32` (or `` if unit)
    fn return_ty<'ast>(&'ast self) -> ReturnTy<'ast> {
        ReturnTy { sig: self }
    }
}

struct ArgDecls<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgDecls<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = &self.sig.args;
        quote_in!(tokens, #(#args),*);
    }
}

struct ArgNames<'ast> {
    sig: &'ast FnSig
}

impl<'ast> ToTokens for ArgNames<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let args = self.sig.args.iter().map(|arg| arg.name);
        quote_in!(tokens, #(#args),*);
    }
}

struct ReturnTy<'ast> {
    sig: &'ast FnSig,
}

impl<'ast> ToTokens for ReturnTy<'ast> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if let Some(ref return_ty) = self.sig.return_ty {
            quote_in!(tokens, -> #return_ty)
        }
    }
}

impl Path {
    fn ty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: false, path: self }
    }

    fn exprty<'a>(&'a self) -> SepPath<'a> {
        SepPath { cc: true, path: self }
    }
}

struct SepPath<'a> {
    cc: bool,
    path: &'a Path,
}

impl<'a> ToTokens for SepPath<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self.path {
            Path::FromRoot => tokens.append("::"),
            Path::FromSelf => tokens.append("self"),
            Path::FromSuper => tokens.append("super"),
            Path::FromTraitItem(ref i) => i.to_tokens(tokens),
            Path::From(ref i) => {
                let i = SepPathId { cc: self.cc, path_id: i};
                i.to_tokens(tokens)
            }
            Path::Extend(ref b, ref i) => {
                let b = SepPath { cc: self.cc, path: b };
                let i = SepPathId { cc: self.cc, path_id: i };
                quote_in!(tokens, #b :: #i)
            }
        }
    }
}

struct SepPathId<'a> {
    cc: bool,
    path_id: &'a PathId,
}

impl<'a> ToTokens for SepPathId<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.path_id.name.to_tokens(tokens);
        let tys = &self.path_id.tys;
        if !tys.is_empty() {
            if self.cc {
                tokens.append("::");
            }
            quote_in!(tokens, <#(#tys),*>);
        }
    }
}

impl ToTokens for TraitItemId {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let &TraitItemId { ref self_ty, ref trait_ref, item } = self;
        let trait_ref = trait_ref.ty();
        quote_in!(tokens, < #self_ty as #trait_ref > :: #item);
    }
}
