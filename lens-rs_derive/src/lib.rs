#![allow(dead_code)]
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::*;
use syn::{
    parenthesized, parse_macro_input, parse_quote, visit::Visit, Data, DeriveInput, Fields,
    ItemEnum, ItemStruct, Token,
};

use crate::meta_impls::*;
use std::collections::HashSet;
use std::fs;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

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
    var.attrs.clone().into_iter().find(|attr: &syn::Attribute| {
        attr.path
            .is_ident(&syn::Ident::new("optic", Span::call_site()))
    })
}

fn field_with_optic_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        attr.path
            .is_ident(&syn::Ident::new("optic", Span::call_site()))
    })
}

fn field_optic_attr(var: &syn::Field) -> Option<syn::Attribute> {
    var.attrs.clone().into_iter().find(|attr: &syn::Attribute| {
        attr.path
            .is_ident(&syn::Ident::new("optic", Span::call_site()))
    })
}

#[proc_macro_derive(Optic, attributes(optic))]
pub fn derive_optic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let optics: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fs),
            ..
        }) => fs
            .named
            .iter()
            .filter(|field| field_with_optic_attr(field))
            .flat_map(|f| {
                impl_optic4field(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    f.ident.clone().unwrap(),
                    f.ty.clone(),
                )
            })
            .collect(),
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fs),
            ..
        }) => fs
            .unnamed
            .iter()
            .take(7)
            .filter(|field| field_with_optic_attr(field))
            .enumerate()
            .flat_map(|(i, f)| {
                impl_optic4index(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    syn::Index::from(i),
                    f.ty.clone(),
                )
            })
            .collect(),
        Data::Enum(e) => e
            .variants
            .iter()
            .filter(|v| variant_with_optic_attr(v))
            .flat_map(|var| match var.fields.clone() {
                Fields::Unnamed(fs) if fs.unnamed.len() == 1 => impl_optic4variant(
                    derive_input.ident.clone(),
                    derive_input.generics.clone(),
                    var.ident.clone(),
                    fs.unnamed[0].ty.clone(),
                ),
                _ => panic!("can only derive `Optic` for variant with single, unnamed variant"),
            })
            .collect(),
        _ => panic!("union  can't derive the `Optic`"),
    };

    TokenStream::from(optics)
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
                _ => panic!("can only derive `Prism` for variant with single, unnamed variant"),
            })
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
            .collect(),
        Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fs),
            ..
        }) => fs
            .unnamed
            .iter()
            .take(7)
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
            .collect(),
        _ => panic!("union and enum can't derive the `Lens`"),
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
        ix: syn::Type,
    },
}

#[derive(Clone, Debug)]
struct OpticsPathType {
    path: Punctuated<AnOpticType, Token![.]>,
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
                ix: content.parse()?,
            })
        } else {
            Ok(AnOpticType::Custom(input.parse()?))
        }
    }
}

impl Parse for OpticsPathType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(OpticsPathType {
            path: Punctuated::parse_terminated(input)?,
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

struct OpticCollector<'a>(&'a mut OpticMap);

impl<'a> OpticCollector<'a> {
    fn collect_optic_fields<'f>(&mut self, fields: impl Iterator<Item = &'f syn::Field>) {
        fields.for_each(|field| {
            if field_with_optic_attr(field) {
                field
                    .ident
                    .as_ref()
                    .map(|ident| ident.to_string())
                    .map(|optic_name| self.0.insert(optic_name));
            }
        });
    }
}

impl<'a> Visit<'_> for OpticCollector<'a> {
    fn visit_item_enum(&mut self, item_enum: &ItemEnum) {
        item_enum.variants.iter().for_each(|variant| {
            if variant_with_optic_attr(variant) {
                self.0.insert(format!("{}", variant.ident));
            }
        })
    }

    fn visit_item_struct(&mut self, item_struct: &ItemStruct) {
        match &item_struct.fields {
            syn::Fields::Named(fields_named) => {
                self.collect_optic_fields(fields_named.named.iter())
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                self.collect_optic_fields(fields_unnamed.unnamed.iter())
            }
            syn::Fields::Unit => (),
        }
    }
}

type OpticMap = HashSet<String>;

#[doc(hidden)]
#[proc_macro]
pub fn scan_optics_from_source_files(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    let mut optcis_map = OpticMap::new();

    loop {
        let token_tree = iter.next();
        match token_tree {
            Some(TokenTree::Literal(literal)) => {
                let file_name = literal.to_string();
                let file_name = file_name.trim_matches('"');
                let contents =
                    String::from_utf8(fs::read(std::path::Path::new(file_name)).unwrap()).unwrap();
                let syntax = syn::parse_file(&contents)
                    .expect(".rs files should contain valid Rust source code.");
                OpticCollector(&mut optcis_map).visit_file(&syntax);

                if let Some(token_tree) = iter.next() {
                    if let TokenTree::Punct(punct) = token_tree {
                        if punct.to_string() != "," {
                            panic!(
                                "scan_optics_from_source_files!(): expect `,`, got `{}`",
                                punct.to_string()
                            );
                        }
                    }
                } else {
                    break;
                }
            }
            None => break,
            _ => panic!(
                "scan_optics_from_source_files!(): expect string literal, got `{:?}`",
                token_tree
            ),
        }
    }

    let mut struct_items = Vec::<ItemStruct>::with_capacity(optcis_map.len());

    for optic_name in optcis_map {
        if let "Some" | "None" | "Ok" | "Err" = &*optic_name { continue; }
        let optic_ident = syn::Ident::new(&optic_name, Span::call_site());
        struct_items.push(parse_quote! {

               #[derive(Copy, Clone, Debug, Eq, PartialEq)]
               #[allow(non_camel_case_types)]
               pub struct #optic_ident<Optics>(pub Optics);
        });
    }

    quote!( #( #struct_items )* ).into()
}
