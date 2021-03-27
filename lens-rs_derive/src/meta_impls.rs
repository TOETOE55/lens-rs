extern crate proc_macro;
use proc_macro2::Span;
use quote::*;
use syn::{
    parse_quote, punctuated::Punctuated, Token
};

#[derive(Clone, Debug)]
struct Type {
    name: syn::Ident,
    lt_token: Option<Token![<]>,
    params: Punctuated<syn::Ident, Token![,]>,
    gt_token: Option<Token![>]>,
}

impl Type {
    fn new(ty_name: syn::Ident, generic: syn::Generics) -> Self {
        let params = generic
            .params
            .into_iter()
            .map(|param| match param {
                syn::GenericParam::Type(syn::TypeParam { ident, .. })
                | syn::GenericParam::Lifetime(syn::LifetimeDef {
                    lifetime: syn::Lifetime { ident, .. },
                    ..
                })
                | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => ident,
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
    fn new(generic: syn::Generics, optics_bound: syn::WherePredicate) -> Self {
        let constraints = generic
            .params
            .into_iter()
            .flat_map(|param| match param {
                syn::GenericParam::Type(ty) => ty.colon_token.map(|colon_token| {
                    syn::WherePredicate::Type(syn::PredicateType {
                        lifetimes: None,
                        bounded_ty: syn::Type::Verbatim(ty.ident.to_token_stream()),
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
            .chain(Some(optics_bound))
            .collect::<Punctuated<syn::WherePredicate, Token![,]>>();

        Self { constraints }
    }
}

impl ToTokens for Constraints {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.constraints.to_tokens(tokens)
    }
}

struct Params {
    lt_token: Token![<],
    params: Punctuated<syn::GenericParam, Token![,]>,
    gt_token: Token![>],
}

impl Params {
    fn new(generic: syn::Generics, optics_param: syn::Ident) -> Self {
        let mut params = Punctuated::new();

        let mut added = false;
        for p in generic.params {
            match p {
                syn::GenericParam::Type(ty) => {
                    if !added {
                        params.push(syn::GenericParam::Type(syn::TypeParam {
                            attrs: vec![],
                            ident: optics_param.clone(),
                            colon_token: None,
                            bounds: Default::default(),
                            eq_token: None,
                            default: None,
                        }));
                        added = true;
                    }
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

pub fn impl_review4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let optics_trait = syn::Ident::new("Review", Span::call_site());
    let rv_param = syn::Ident::new("__Rv", Span::call_site());

    // <...>
    let params = Params::new(generic.clone(), rv_param.clone());

    // ty<...>
    let ty = Type::new(ty_name.clone(), generic.clone());

    // where ...
    let optics_bound = parse_quote! { #field_ty: lens_rs::#optics_trait<#rv_param> };
    let constraints = Constraints::new(generic.clone(), optics_bound);

    quote! {
        impl #params lens_rs::#optics_trait<#var_name<#rv_param>> for #ty
        where
            #constraints
        {
            type From = <#field_ty>::From;
            fn review(optics: Optic, from: Self::From) -> Self {
                <#ty>::#var_name(Review::review(optics.0, from))
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
    quote! {}
}

pub fn impl_mut4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_mv4variant(
    ty_name: syn::Ident,
    generic: syn::Generics,

    var_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_ref4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_mut4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_mv4field(
    ty_name: syn::Ident,
    generic: syn::Generics,

    field_name: syn::Ident,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_ref4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    index: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_mut4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    index: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}

pub fn impl_mv4index(
    ty_name: syn::Ident,
    generic: syn::Generics,

    index: syn::Index,
    field_ty: syn::Type,
) -> proc_macro2::TokenStream {
    quote! {}
}