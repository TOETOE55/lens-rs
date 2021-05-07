extern crate proc_macro;
use proc_macro2::{Span, TokenStream};
use quote::*;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_quote,
    punctuated::Punctuated,
    GenericParam, Token,
};

pub fn impl_review4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Review", Span::call_site());
    let rv_param = syn::Ident::new("__Rv", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(generic.clone(), vec![rv_param.clone(), image_param.clone()]);

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bound = parse_quote! { #field_ty: lens_rs::#optics_trait<#rv_param, #image_param> };
    let constraints = Constraints::new(generic, vec![optics_bound]);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#rv_param>, #image_param> for #ty
        where
            #constraints
        {
            fn review(optics: lens_rs::optics::#var_name<#rv_param>, from: #image_param) -> Self {
                <#ty>::#var_name(lens_rs::Review::review(optics.0, from))
            }
        }
    }
}

pub fn impl_ref4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let traversal_impl = impl_traversal_ref4variant(
        ty_name.clone(),
        generic.clone(),
        var_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_ref4variant(ty_name, generic, var_name, field_ty);
    quote! {
        #traversal_impl
        #prism_impl
    }
}

pub fn impl_mut4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let rf = impl_ref4variant(
        ty_name.clone(),
        generic.clone(),
        var_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal_mut4variant(
        ty_name.clone(),
        generic.clone(),
        var_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_mut4variant(ty_name, generic, var_name, field_ty);
    quote! {
        #rf
        #traversal_impl
        #prism_impl
    }
}

pub fn impl_mv4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let mt = impl_mut4variant(
        ty_name.clone(),
        generic.clone(),
        var_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal4variant(
        ty_name.clone(),
        generic.clone(),
        var_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism4variant(ty_name, generic, var_name, field_ty);
    quote! {
        #mt
        #traversal_impl
        #prism_impl
    }
}

pub fn impl_ref4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let traversal_impl = impl_traversal_ref4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_ref4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens_ref4field(ty_name, generic, field_name, field_ty);

    quote! {
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

pub fn impl_mut4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let rf = impl_ref4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal_mut4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_mut4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens_mut4field(ty_name, generic, field_name, field_ty);

    quote! {
        #rf
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

pub fn impl_mv4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let mt = impl_mut4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism4field(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens4field(ty_name, generic, field_name, field_ty);

    quote! {
        #mt
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

pub fn impl_ref4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let traversal_impl = impl_traversal_ref4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_ref4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens_ref4index(ty_name, generic, field_name, field_ty);

    quote! {
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

pub fn impl_mut4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let rf = impl_ref4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal_mut4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism_mut4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens_mut4index(ty_name, generic, field_name, field_ty);

    quote! {
        #rf
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

pub fn impl_mv4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let mt = impl_mut4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let traversal_impl = impl_traversal4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let prism_impl = impl_prism4index(
        ty_name.clone(),
        generic.clone(),
        field_name.clone(),
        field_ty.clone(),
    );
    let lens_impl = impl_lens4index(ty_name, generic, field_name, field_ty);

    quote! {
        #mt
        #traversal_impl
        #prism_impl
        #lens_impl
    }
}

#[derive(Clone, Debug)]
enum Param {
    Ident(syn::Ident),
    Lifetime(syn::LifetimeDef),
}

impl ToTokens for Param {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Param::Ident(id) => id.to_tokens(tokens),
            Param::Lifetime(life) => life.to_tokens(tokens),
        }
    }
}

#[derive(Clone, Debug)]
struct Type {
    name: syn::Ident,
    lt_token: Option<Token![<]>,
    params: Punctuated<Param, Token![,]>,
    gt_token: Option<Token![>]>,
}

impl Type {
    fn new(ty_name: syn::Ident, generic: syn::Generics) -> Self {
        let params = generic
            .params
            .into_iter()
            .map(|param| match param {
                syn::GenericParam::Type(syn::TypeParam { ident, .. })
                | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => Param::Ident(ident),
                syn::GenericParam::Lifetime(life) => Param::Lifetime(life),
            })
            .collect();
        Self {
            name: ty_name,
            lt_token: generic.lt_token,
            params,
            gt_token: generic.gt_token,
        }
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.lt_token.to_tokens(tokens);
        self.params.to_tokens(tokens);
        self.gt_token.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
struct Constraints {
    constraints: Punctuated<syn::WherePredicate, Token![,]>,
}

impl Constraints {
    fn new(generic: syn::Generics, optics_bounds: Vec<syn::WherePredicate>) -> Self {
        let constraints = generic
            .params
            .into_iter()
            .flat_map(|param| match param {
                syn::GenericParam::Type(ty) => ty.colon_token.map(|colon_token| {
                    syn::WherePredicate::Type(syn::PredicateType {
                        lifetimes: None,
                        bounded_ty: syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments: {
                                    let mut seg = Punctuated::new();
                                    seg.push(syn::PathSegment {
                                        ident: ty.ident.clone(),
                                        arguments: Default::default(),
                                    });
                                    seg
                                },
                            },
                        }),
                        colon_token,
                        bounds: ty.bounds,
                    })
                }),
                syn::GenericParam::Lifetime(life) => life.colon_token.map(|colon_token| {
                    syn::WherePredicate::Lifetime(syn::PredicateLifetime {
                        lifetime: life.lifetime,
                        colon_token,
                        bounds: life.bounds,
                    })
                }),
                syn::GenericParam::Const(_) => None,
            })
            .chain(generic.where_clause.into_iter().flat_map(|w| w.predicates))
            .chain(optics_bounds)
            .collect::<Punctuated<syn::WherePredicate, Token![,]>>();

        Self { constraints }
    }
}

impl ToTokens for Constraints {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.constraints.to_tokens(tokens)
    }
}

#[derive(Clone, Debug)]
struct Params {
    lt_token: Token![<],
    params: Punctuated<syn::GenericParam, Token![,]>,
    gt_token: Token![>],
}

impl Params {
    fn new(generics: syn::Generics, other_param: Vec<syn::Ident>) -> Self {
        let mut params = Punctuated::new();
        let mut generics_iter = generics.params.into_iter();

        let mut added = false;
        while let Some(generic) = generics_iter.next() {
            match generic {
                syn::GenericParam::Lifetime(lifetime) => {
                    params.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
                        attrs: vec![],
                        lifetime: lifetime.lifetime,
                        colon_token: None,
                        bounds: Default::default(),
                    }));
                }
                GenericParam::Type(ty) => {
                    for param in other_param.iter() {
                        params.push(syn::GenericParam::Type(syn::TypeParam {
                            attrs: vec![],
                            ident: param.clone(),
                            colon_token: None,
                            bounds: Default::default(),
                            eq_token: None,
                            default: None,
                        }));
                    }

                    added = true;
                    params.push(syn::GenericParam::Type(syn::TypeParam {
                        attrs: vec![],
                        ident: ty.ident,
                        colon_token: None,
                        bounds: Default::default(),
                        eq_token: None,
                        default: None,
                    }));
                    break;
                }
                c @ GenericParam::Const(_) => {
                    for param in other_param.iter() {
                        params.push(syn::GenericParam::Type(syn::TypeParam {
                            attrs: vec![],
                            ident: param.clone(),
                            colon_token: None,
                            bounds: Default::default(),
                            eq_token: None,
                            default: None,
                        }));
                    }
                    added = true;
                    params.push(c);
                    break;
                }
            }
        }

        if !added {
            for param in other_param {
                params.push(syn::GenericParam::Type(syn::TypeParam {
                    attrs: vec![],
                    ident: param,
                    colon_token: None,
                    bounds: Default::default(),
                    eq_token: None,
                    default: None,
                }));
            }
        }

        for p in generics_iter {
            match p {
                syn::GenericParam::Type(ty) => {
                    params.push(syn::GenericParam::Type(syn::TypeParam {
                        attrs: vec![],
                        ident: ty.ident,
                        colon_token: None,
                        bounds: Default::default(),
                        eq_token: None,
                        default: None,
                    }));
                }

                syn::GenericParam::Lifetime(lifetime) => {
                    params.push(syn::GenericParam::Lifetime(syn::LifetimeDef {
                        attrs: vec![],
                        lifetime: lifetime.lifetime,
                        colon_token: None,
                        bounds: Default::default(),
                    }))
                }

                c @ syn::GenericParam::Const(_) => params.push(c),
            }
        }

        // println!("{}", quote! { #params }.to_string());
        Self {
            lt_token: syn::token::Lt {
                spans: [Span::call_site()],
            },
            params,
            gt_token: syn::token::Gt {
                spans: [Span::call_site()],
            },
        }
    }
}

impl ToTokens for Params {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.lt_token.to_tokens(tokens);
        self.params.to_tokens(tokens);
        self.gt_token.to_tokens(tokens);
    }
}

fn impl_traversal_ref4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalRef", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_ref(&self, optics: lens_rs::optics::#var_name<#traversal_param>) -> Vec<&#image_param> {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_ref(x, optics.0),
                     _ => vec![],
                }
            }
        }
    }
}

fn impl_prism_ref4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismRef", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_ref(&self, optics: lens_rs::optics::#var_name<#prism_param>) -> Option<&#image_param> {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_ref(x, optics.0),
                     _ => Option::None,
                }
            }
        }
    }
}

fn impl_traversal_mut4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalMut", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_mut(&mut self, optics: lens_rs::optics::#var_name<#traversal_param>) -> Vec<&mut #image_param> {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_mut(x, optics.0),
                     _ => vec![],
                }
            }
        }
    }
}

fn impl_prism_mut4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismMut", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_mut(&mut self, optics: lens_rs::optics::#var_name<#prism_param>) -> Option<&mut #image_param> {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_mut(x, optics.0),
                     _ => Option::None,
                }
            }
        }
    }
}

fn impl_traversal4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Traversal", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse(self, optics: lens_rs::optics::#var_name<#traversal_param>) -> Vec<#image_param>
            where
                Self: Sized,
            {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse(x, optics.0),
                     _ => vec![],
                }
            }
        }
    }
}

fn impl_prism4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Prism", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#prism_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#var_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview(self, optics: lens_rs::optics::#var_name<#prism_param>) -> Option<#image_param>
            where
                Self: Sized,
            {
                use #ty_name::*;
                match self {
                    #var_name(x) => <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview(x, optics.0),
                     _ => Option::None,
                }
            }
        }
    }
}

fn impl_traversal_ref4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalRef", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_ref(&self, optics: lens_rs::optics::#field_name<#traversal_param>) -> Vec<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_ref4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismRef", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_ref(&self, optics: lens_rs::optics::#field_name<#prism_param>) -> Option<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_ref4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensRef", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view_ref(&self, optics: lens_rs::optics::#field_name<#lens_param>) -> & #image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal_mut4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalMut", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_mut(&mut self, optics: lens_rs::optics::#field_name<#traversal_param>) -> Vec<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_mut4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismMut", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_mut(&mut self, optics: lens_rs::optics::#field_name<#prism_param>) -> Option<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_mut4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensMut", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view_mut(&mut self, optics: lens_rs::optics::#field_name<#lens_param>) -> &mut #image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Traversal", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse(self, optics: lens_rs::optics::#field_name<#traversal_param>) -> Vec<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Prism", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#prism_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview(self, optics: lens_rs::optics::#field_name<#prism_param>) -> Option<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Lens", Span::call_site());
    let prism_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#prism_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#field_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view(self, optics: lens_rs::optics::#field_name<#prism_param>) -> #image_param
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::view(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal_ref4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalRef", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_ref(&self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_ref4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismRef", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_ref(&self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_ref4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensRef", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view_ref(&self, optics: lens_rs::optics::#optics_name<#lens_param>) -> &#image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal_mut4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalMut", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse_mut(&mut self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_mut4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismMut", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview_mut(&mut self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_mut4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensMut", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds = vec![
        parse_quote! {
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        },
        parse_quote! {
            #image_param: ?Sized
        },
    ];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view_mut(&mut self, optics: lens_rs::optics::#optics_name<#lens_param>) -> &mut #image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Traversal", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![traversal_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse(self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Prism", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![prism_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#prism_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview(self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Lens", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let optics_name = format_ident!("_{}", field_name);

    // <...>
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    // ty<...>
    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let optics_bounds =
        vec![parse_quote! { #field_ty: lens_rs::#optics_trait<#lens_param, #image_param> }];
    let constraints = Constraints::new(generic, optics_bounds);

    quote! {
        impl #params lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn view(self, optics: lens_rs::optics::#optics_name<#lens_param>) -> #image_param
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view(self.#field_name, optics.0)
            }
        }
    }
}

pub fn impl_empty(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
) -> proc_macro2::TokenStream {
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let lens_param = syn::Ident::new("__Opt", Span::call_site());
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let constraints = Constraints::new(generic, vec![]);

    quote! {
        impl #params lens_rs::TraversalRef<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #image_param: ?Sized,
            #constraints

        {
            #[inline] fn traverse_ref(&self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<& #image_param> {
                vec![]
            }
        }

        impl #params lens_rs::TraversalMut<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #image_param: ?Sized,
            #constraints

        {
            #[inline] fn traverse_mut(&mut self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<&mut #image_param> {
                vec![]
            }
        }

        impl #params lens_rs::Traversal<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn traverse(self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<#image_param>
            where
                Self: Sized,
            {
                vec![]
            }
        }

        impl #params lens_rs::PrismRef<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #image_param: ?Sized,
            #constraints
        {
            #[inline] fn preview_ref(&self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<& #image_param> {
                None
            }
        }

        impl #params lens_rs::PrismMut<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #image_param: ?Sized,
            #constraints
        {
            #[inline] fn preview_mut(&mut self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<&mut #image_param> {
                None
            }
        }

        impl #params lens_rs::Prism<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            #[inline] fn preview(self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<#image_param>
            where
                Self: Sized,
            {
                None
            }
        }
    }
}

pub fn impl_empty_review_named(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
) -> proc_macro2::TokenStream {
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let lens_param = syn::Ident::new("__Opt", Span::call_site());
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let constraints = Constraints::new(generic, vec![]);

    quote! {
        impl #params lens_rs::Review<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            fn review(_optics: lens_rs::optics::#field_name<#lens_param >, _from: #image_param) -> Self {
                #ty::#field_name { }
            }
        }
    }
}

pub fn impl_empty_review_unnamed(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
) -> proc_macro2::TokenStream {
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let lens_param = syn::Ident::new("__Opt", Span::call_site());
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let constraints = Constraints::new(generic, vec![]);

    quote! {
        impl #params lens_rs::Review<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            fn review(_optics: lens_rs::optics::#field_name<#lens_param >, _from: #image_param) -> Self {
                <#ty>::#field_name()
            }
        }
    }
}

pub fn impl_empty_review_unit(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
) -> proc_macro2::TokenStream {
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let lens_param = syn::Ident::new("__Opt", Span::call_site());
    let params = Params::new(
        generic.clone(),
        vec![lens_param.clone(), image_param.clone()],
    );

    let ty = Type::new(ty_name, generic.clone());

    // where ...
    let constraints = Constraints::new(generic, vec![]);

    quote! {
        impl #params lens_rs::Review<lens_rs::optics::#field_name<#lens_param >, #image_param> for #ty
        where
            #constraints
        {
            fn review(_optics: lens_rs::optics::#field_name<#lens_param >, _from: #image_param) -> Self {
                <#ty>::#field_name
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tuple {
    pub _paren_token: syn::token::Paren,
    pub elems: Punctuated<syn::Ident, Token![,]>,
}

impl Parse for Tuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren_token = syn::parenthesized!(content in input);
        Ok(Self {
            _paren_token,
            elems: Punctuated::parse_terminated(&content)?,
        })
    }
}

pub fn impl4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let traversal_ref =
        impl_traversal_ref4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let prism_ref = impl_prism_ref4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let lens_ref = impl_lens_ref4tuple(tuple.clone(), field_name.clone(), field_ty.clone());

    let traversal_mut =
        impl_traversal_mut4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let prism_mut = impl_prism_mut4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let lens_mut = impl_lens_mut4tuple(tuple.clone(), field_name.clone(), field_ty.clone());

    let traversal = impl_traversal4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let prism = impl_prism4tuple(tuple.clone(), field_name.clone(), field_ty.clone());
    let lens = impl_lens4tuple(tuple, field_name, field_ty);

    quote! {
        #traversal_ref
        #traversal_mut
        #traversal

        #prism_ref
        #prism_mut
        #prism

        #lens_ref
        #lens_mut
        #lens

    }
}

fn impl_traversal_ref4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalRef", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #traversal_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        {
            #[inline] fn traverse_ref(&self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_ref4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismRef", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #prism_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        {
            #[inline] fn preview_ref(&self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<&#image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_ref4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensRef", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #lens_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        {
            #[inline] fn view_ref(&self, optics: lens_rs::optics::#optics_name<#lens_param>) -> &#image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_ref(&self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal_mut4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("TraversalMut", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #traversal_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        {
            #[inline] fn traverse_mut(&mut self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism_mut4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("PrismMut", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #prism_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        {
            #[inline] fn preview_mut(&mut self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<&mut #image_param> {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens_mut4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("LensMut", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #lens_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        {
            #[inline] fn view_mut(&mut self, optics: lens_rs::optics::#optics_name<#lens_param>) -> &mut #image_param {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view_mut(&mut self.#field_name, optics.0)
            }
        }
    }
}

fn impl_traversal4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Traversal", Span::call_site());
    let traversal_param = syn::Ident::new("__Tr", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #traversal_param, #params> lens_rs::#optics_trait<lens_rs::optics::#optics_name<#traversal_param>, #image_param> for (#params)
        where
            #field_ty: lens_rs::#optics_trait<#traversal_param, #image_param>
        {
            #[inline] fn traverse(self, optics: lens_rs::optics::#optics_name<#traversal_param>) -> Vec<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#traversal_param, #image_param>>::traverse(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_prism4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Prism", Span::call_site());
    let prism_param = syn::Ident::new("__Pm", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #prism_param, #params>  lens_rs::#optics_trait<lens_rs::optics::#optics_name<#prism_param>, #image_param> for (#params)
        where
            #field_ty: lens_rs::#optics_trait<#prism_param, #image_param>
        {
            #[inline] fn preview(self, optics: lens_rs::optics::#optics_name<#prism_param>) -> Option<#image_param>
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#prism_param, #image_param>>::preview(self.#field_name, optics.0)
            }
        }
    }
}

fn impl_lens4tuple(
    tuple: crate::Tuple,

    field_name: syn::Index,
    field_ty: syn::Ident,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Lens", Span::call_site());
    let lens_param = syn::Ident::new("__Ls", Span::call_site());
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let params = &tuple.elems;
    let optics_name = format_ident!("_{}", field_name);

    quote! {
        impl<#image_param, #lens_param, #params>  lens_rs::#optics_trait<lens_rs::optics::#optics_name<#lens_param >, #image_param> for (#params)
        where
            #field_ty: lens_rs::#optics_trait<#lens_param, #image_param>
        {
            #[inline] fn view(self, optics: lens_rs::optics::#optics_name<#lens_param>) -> #image_param
            where
                Self: Sized,
            {
                <#field_ty as lens_rs::#optics_trait<#lens_param, #image_param>>::view(self.#field_name, optics.0)
            }
        }
    }
}

pub fn impl_empty4tuple(tuple: crate::Tuple, field_name: syn::Ident) -> proc_macro2::TokenStream {
    let image_param = syn::Ident::new("__Image", Span::call_site());
    let lens_param = syn::Ident::new("__Opt", Span::call_site());
    let params = &tuple.elems;

    quote! {
        impl<#image_param, #lens_param, #params> lens_rs::TraversalRef<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,

        {
            #[inline] fn traverse_ref(&self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<& #image_param> {
                vec![]
            }
        }

        impl<#image_param, #lens_param, #params> lens_rs::TraversalMut<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,

        {
            #[inline] fn traverse_mut(&mut self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<&mut #image_param> {
                vec![]
            }
        }

        impl<#image_param, #lens_param, #params> lens_rs::Traversal<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        {
            #[inline] fn traverse(self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Vec<#image_param>
            where
                Self: Sized,
            {
                vec![]
            }
        }

        impl<#image_param, #lens_param, #params> lens_rs::PrismRef<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,
        {
            #[inline] fn preview_ref(&self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<& #image_param> {
                None
            }
        }

        impl<#image_param, #lens_param, #params> lens_rs::PrismMut<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        where
            #image_param: ?Sized,
        {
            #[inline] fn preview_mut(&mut self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<&mut #image_param> {
                None
            }
        }

        impl<#image_param, #lens_param, #params> lens_rs::Prism<lens_rs::optics::#field_name<#lens_param >, #image_param> for (#params)
        {
            #[inline] fn preview(self, _optics: lens_rs::optics::#field_name<#lens_param >) -> Option<#image_param>
            where
                Self: Sized,
            {
                None
            }
        }
    }
}
