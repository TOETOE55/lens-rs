extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Fields, Token,
};

use crate::meta_impls::*;
use lens_rs_generator::all_optics;

mod meta_impls;

enum OpticMutability {
    Move,
    Ref(Token![ref]),
    Mut(Token![mut]),
}

impl Parse for OpticMutability {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self::Move);
        }

        let content;

        parenthesized!(content in input);
        let lookahead = content.lookahead1();
        if lookahead.peek(Token![mut]) {
            Ok(Self::Mut(content.parse()?))
        } else if lookahead.peek(Token![ref]) {
            Ok(Self::Ref(content.parse()?))
        } else {
            Err(input.error("only allow #[optic], #[optic(mut)] or #[optic(ref)] here"))
        }
    }
}

fn variant_with_optic_attr(var: &syn::Variant) -> bool {
    var.attrs.iter().any(|attr| {
        attr.path
            .is_ident(&syn::Ident::new("optic", Span::call_site()))
    })
}

fn variant_optic_attr(var: &syn::Variant) -> Option<syn::Attribute> {
    var.attrs
        .iter()
        .find(|attr: &&syn::Attribute| {
            attr.path
                .is_ident(&syn::Ident::new("optic", Span::call_site()))
        })
        .cloned()
}

fn field_with_optic_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path
            .is_ident(&syn::Ident::new("optic", Span::call_site()))
    })
}

fn field_optic_attr(var: &syn::Field) -> Option<syn::Attribute> {
    var.attrs
        .iter()
        .find(|attr: &&syn::Attribute| {
            attr.path
                .is_ident(&syn::Ident::new("optic", Span::call_site()))
        })
        .cloned()
}

#[proc_macro_derive(Review, attributes(optic))]
pub fn derive_review(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let reviews: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .filter(|v| variant_with_optic_attr(v))
            .flat_map(|var| match var.fields.clone() {
                Fields::Unnamed(fs) if fs.unnamed.len() == 1 => impl_review4variant(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                    fs.unnamed[0].ty.clone(),
                ),
                Fields::Unnamed(fs) if fs.unnamed.is_empty() => impl_empty_review_unnamed(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                Fields::Named(fs) if fs.named.is_empty() => impl_empty_review_named(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                Fields::Unit => impl_empty_review_unit(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                _ => panic!("can only derive `Review` for variant with single, unnamed variant"),
            })
            .collect(),
        _ => panic!("union and struct can't derive the `Review`"),
    };
    TokenStream::from(reviews)
}

#[proc_macro_derive(Prism, attributes(optic))]
pub fn derive_prism(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let prisms: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .filter(|var| variant_with_optic_attr(var))
            .flat_map(|var| match var.fields.clone() {
                Fields::Unnamed(fs) if fs.unnamed.len() == 1 => {
                    let attr: syn::Attribute = variant_optic_attr(var).unwrap();
                    let mutability =
                        syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();
                    match mutability {
                        OpticMutability::Ref(_) => impl_ref4variant(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            var.ident.clone(),
                            fs.unnamed[0].ty.clone(),
                        ),
                        OpticMutability::Mut(_) => impl_mut4variant(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            var.ident.clone(),
                            fs.unnamed[0].ty.clone(),
                        ),
                        OpticMutability::Move => impl_mv4variant(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            var.ident.clone(),
                            fs.unnamed[0].ty.clone(),
                        ),
                    }
                }
                Fields::Unnamed(fs) if fs.unnamed.is_empty() => impl_empty(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                Fields::Named(fs) if fs.named.is_empty() => impl_empty(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                Fields::Unit => impl_empty(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                ),
                _ => panic!(
                    "can only derive `Prism` for variant with single and unnamed, or unit variant"
                ),
            })
            .chain(
                all_optics()
                    .into_iter()
                    .filter(|ident| !e.variants.iter().any(|v| &v.ident == ident))
                    .flat_map(|ident| {
                        impl_empty(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            ident,
                        )
                    }),
            )
            .collect(),
        _ => panic!("union and struct can't derive the `Prism`"),
    };

    TokenStream::from(prisms)
}

#[proc_macro_derive(Lens, attributes(optic))]
pub fn derive_lens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let lens: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fs),
            ..
        }) => fs
            .named
            .iter()
            .filter(|field| field_with_optic_attr(field))
            .flat_map(|f| {
                let attr: syn::Attribute = field_optic_attr(f).unwrap();
                let mutability =
                    syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();
                match mutability {
                    OpticMutability::Ref(_) => impl_ref4field(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        f.ident.clone().unwrap(),
                        f.ty.clone(),
                    ),
                    OpticMutability::Mut(_) => impl_mut4field(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        f.ident.clone().unwrap(),
                        f.ty.clone(),
                    ),
                    OpticMutability::Move => impl_mv4field(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        f.ident.clone().unwrap(),
                        f.ty.clone(),
                    ),
                }
            })
            .chain(
                all_optics()
                    .into_iter()
                    .filter(|ident| !fs.named.iter().any(|f| f.ident.as_ref().unwrap() == ident))
                    .flat_map(|ident| {
                        impl_empty(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            ident,
                        )
                    }),
            )
            .collect(),
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fs),
            ..
        }) => fs
            .unnamed
            .iter()
            .take(17)
            .filter(|field| field_with_optic_attr(field))
            .enumerate()
            .flat_map(|(i, f)| {
                let attr: syn::Attribute = field_optic_attr(f).unwrap();
                let mutability =
                    syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();
                let field_name = syn::Index::from(i);
                match mutability {
                    OpticMutability::Ref(_) => impl_ref4index(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        field_name,
                        f.ty.clone(),
                    ),
                    OpticMutability::Mut(_) => impl_mut4index(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        field_name,
                        f.ty.clone(),
                    ),
                    OpticMutability::Move => impl_mv4index(
                        derive_input.ident.clone(),
                        derive_input.generics.clone(),
                        field_name,
                        f.ty.clone(),
                    ),
                }
            })
            .chain(
                all_optics()
                    .into_iter()
                    .filter(|ident| {
                        !fs.unnamed
                            .iter()
                            .enumerate()
                            .any(|(i, _)| format!("_{}", i) == ident.to_string())
                    })
                    .flat_map(|ident| {
                        impl_empty(
                            derive_input.ident.clone(),
                            derive_input.generics.clone(),
                            ident,
                        )
                    }),
            )
            .collect(),
        Data::Struct(_) => all_optics()
            .into_iter()
            .flat_map(|ident| {
                impl_empty(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    ident,
                )
            })
            .collect(),
        _ => panic!("can only derive `Lens` for struct"),
    };

    TokenStream::from(lens)
}

#[derive(Clone, Debug)]
enum AnOpticExpr {
    Default(syn::Ident),
    Custom(syn::Path),
    Ix {
        _bracket_token: syn::token::Bracket,
        ix: syn::Expr,
    },
}

#[derive(Clone, Debug)]
struct OpticsPathExpr {
    path: Punctuated<AnOpticExpr, Token![.]>,
}

impl Parse for AnOpticExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Ident) && !input.peek2(Token![::]) {
            Ok(AnOpticExpr::Default(input.parse()?))
        } else if input.peek(syn::token::Bracket) {
            let content;
            let _bracket_token = syn::bracketed!(content in input);
            Ok(AnOpticExpr::Ix {
                _bracket_token,
                ix: content.parse()?,
            })
        } else {
            Ok(AnOpticExpr::Custom(input.parse()?))
        }
    }
}

impl Parse for OpticsPathExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(OpticsPathExpr {
            path: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn optics(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as OpticsPathExpr);
    path.path
        .into_iter()
        .rev()
        .fold(quote! { lens_rs::optics::__ }, |opts, opt| match opt {
            AnOpticExpr::Default(id) => quote! { lens_rs::optics::#id(#opts) },
            AnOpticExpr::Custom(p) => quote! { #p(#opts) },
            AnOpticExpr::Ix { ix, .. } => quote! { lens_rs::optics::_ix(#opts, #ix) },
        })
        .into()
}

#[derive(Clone, Debug)]
enum AnOpticType {
    Default(syn::Ident),
    Custom(syn::Path),
    Ix {
        _bracket_token: syn::token::Bracket,
        _ix: syn::Type,
    },
}

#[derive(Clone, Debug)]
struct OpticsPathType {
    _path: Punctuated<AnOpticType, Token![.]>,
}

impl Parse for AnOpticType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Ident) && !input.peek2(Token![::]) {
            Ok(AnOpticType::Default(input.parse()?))
        } else if input.peek(syn::token::Bracket) {
            let content;
            let _bracket_token = syn::bracketed!(content in input);
            Ok(AnOpticType::Ix {
                _bracket_token,
                _ix: content.parse()?,
            })
        } else {
            Ok(AnOpticType::Custom(input.parse()?))
        }
    }
}

impl Parse for OpticsPathType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(OpticsPathType {
            _path: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn Optics(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as OpticsPathExpr);
    path.path
        .into_iter()
        .rev()
        .fold(quote! { lens_rs::optics::__ }, |opts, opt| match opt {
            AnOpticExpr::Default(id) => quote! { lens_rs::optics::#id<#opts> },
            AnOpticExpr::Custom(p) => quote! { #p::<#opts> },
            AnOpticExpr::Ix { ix, .. } => quote! { lens_rs::optics::_ix<#ix, #opts> },
        })
        .into()
}

#[proc_macro]
#[doc(hidden)]
pub fn derive_review_for_builtin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_review(input)
}

#[proc_macro]
#[doc(hidden)]
pub fn derive_prism_for_builtin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_prism(input)
}

#[proc_macro]
#[doc(hidden)]
pub fn derive_lens_for_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tuple = parse_macro_input!(input as Tuple);
    tuple
        .elems
        .iter()
        .enumerate()
        .flat_map(|(i, id)| {
            let field_name = syn::Index::from(i);
            impl4tuple(tuple.clone(), field_name, id.clone())
        })
        .chain(
            all_optics()
                .into_iter()
                .filter(|ident| {
                    !tuple
                        .elems
                        .iter()
                        .enumerate()
                        .any(|(i, _)| format!("_{}", i) == ident.to_string())
                })
                .flat_map(|ident| impl_empty4tuple(tuple.clone(), ident)),
        )
        .collect::<proc_macro2::TokenStream>()
        .into()
}
