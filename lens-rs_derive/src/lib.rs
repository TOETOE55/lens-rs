extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::*;
use syn::{
    parenthesized, parse_macro_input, parse_quote, punctuated::Punctuated, visit::Visit, Data,
    DeriveInput, ItemEnum, ItemStruct, Token,
};

use std::collections::HashSet;
use std::fs;
use syn::parse::{Parse, ParseStream, Result};

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


#[proc_macro_derive(Review, attributes(optic))]
pub fn derive_review(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let reviews: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .filter(|var| {
                var
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
            })
            .flat_map(|var| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.iter().collect::<Vec<_>>();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let var_name = &var.ident;
                let optic_name = format_ident!("{}", var.ident);
                let ty = var
                    .fields
                    .iter()
                    .take(1)
                    .map(|field| field.ty.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                // let fields = var
                //     .fields
                //     .iter()
                //     .enumerate()
                //     .flat_map(|(i, _)| {
                //         let i = syn::Index::from(i);
                //         quote! { #i }
                //     })
                //     .collect::<Vec<_>>();

                quote! {
                    impl<#(#data_gen_param,)* __Rv> lens_rs::Review<#data_name #data_gen> for lens_rs::optics::#optic_name<__Rv>
                    where
                        __Rv: lens_rs::Review<#ty>,
                        #data_gen_where
                    {
                        type From = __Rv::From;

                        fn review(&self, from: Self::From) -> #data_name #data_gen {
                            // let tuple = self.0.review(from);
                            // <#data_name #data_gen>::#var_name(#(tuple . #fields,)*)
                            <#data_name #data_gen>::#var_name(self.0.review(from))
                        }
                    }
                }
            })
            .collect(),
        _ => panic!("union and struct can't derive the review"),
    };
    TokenStream::from(reviews)
}

#[proc_macro_derive(Prism, attributes(optic))]
pub fn derive_prism(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let prisms: proc_macro2::TokenStream  = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .filter(|var| {
                var
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
            })
            .flat_map(|var| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.iter().collect::<Vec<_>>();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let var_name = &var.ident;
                let optic_name = format_ident!("{}", var.ident);
                let ty = var
                    .fields
                    .iter()
                    .map(|field| field.ty.clone())
                    .take(1)
                    .collect::<Punctuated<_, Token![,]>>();
                let attr: syn::Attribute = var
                    .attrs
                    .clone()
                    .into_iter()
                    .find(|attr: &syn::Attribute| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
                    .unwrap();
                let mutability = syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();



                let impl_ref = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalRef<#data_name #data_gen> for lens_rs::optics::#optic_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalRef<#ty>,
                        #data_gen_where
                    {
                        type To = __Tr::To;

                        fn traverse_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Vec<&'__a98shdai Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.traverse_ref(x),
                                _ => vec![],
                            }
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismRef<#data_name #data_gen> for lens_rs::optics::#optic_name<__Pm>
                    where
                        __Pm: lens_rs::PrismRef<#ty>,
                        #data_gen_where
                    {
                        fn pm_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Option<&'__a98shdai Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.pm_ref(x),
                                _ => Option::None,
                            }
                        }
                    }
                };

                let impl_mut = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalMut<#data_name #data_gen> for lens_rs::optics::#optic_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalMut<#ty>,
                        #data_gen_where
                    {
                        fn traverse_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> Vec<&'__a98shdai mut Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.traverse_mut(x),
                                _ => vec![],
                            }
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismMut<#data_name #data_gen> for lens_rs::optics::#optic_name<__Pm>
                    where
                        __Pm: lens_rs::PrismMut<#ty>,
                        #data_gen_where
                    {
                        fn pm_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> Option<&'__a98shdai mut Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.pm_mut(x),
                                _ => Option::None,
                            }
                        }
                    }
                };

                let impl_mv = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::Traversal<#data_name #data_gen> for lens_rs::optics::#optic_name<__Tr>
                    where
                        __Tr: lens_rs::Traversal<#ty>,
                        #data_gen_where
                    {
                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> where Self::To: Sized {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.traverse(x),
                                _ => vec![],
                            }
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::Prism<#data_name #data_gen> for lens_rs::optics::#optic_name<__Pm>
                    where
                        __Pm: lens_rs::Prism<#ty>,
                        #data_gen_where
                    {
                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> where Self::To: Sized {
                            use #data_name::*;
                            match source {
                                #var_name(x) => self.0.pm(x),
                                _ => Option::None,
                            }
                        }
                    }
                };

                match mutability {
                    OpticMutability::Ref(_) => vec![impl_ref],
                    OpticMutability::Mut(_) => vec![impl_mut, impl_ref],
                    OpticMutability::Move   => vec![impl_mv, impl_mut, impl_ref]
                }.into_iter().flat_map(|x| x)
            })
            .collect(),
        _ => panic!("union and struct can't derive the review"),
    };

    TokenStream::from(prisms)
}

#[proc_macro_derive(Lens, attributes(optic))]
pub fn derive_lens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let lens: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fs), .. }) => fs
            .named
            .iter()
            .filter(|var| {
                var
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
            })
            .flat_map(|f| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.iter().collect::<Vec<_>>();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let optics_name = format_ident!("{}", f.ident.as_ref().unwrap());
                let to = &f.ty;
                let field_name = f.ident.as_ref().unwrap();

                let attr: syn::Attribute = f
                    .attrs
                    .clone()
                    .into_iter()
                    .find(|attr: &syn::Attribute| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
                    .unwrap();
                let mutability = syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();

                let impl_ref = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalRef<#to>,
                        #data_gen_where
                    {
                        type To = __Tr::To;

                        fn traverse_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Vec<&'__a98shdai Self::To> {
                            self.0.traverse_ref(&source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::PrismRef<#to>,
                        #data_gen_where
                    {
                        fn pm_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Option<&'__a98shdai Self::To> {
                            self.0.pm_ref(&source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::LensRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::LensRef<#to>,
                        #data_gen_where
                    {
                        fn view_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> &'__a98shdai Self::To {
                            self.0.view_ref(&source.#field_name)
                        }
                    }
                };

                let impl_mut = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalMut<#to>,
                        #data_gen_where
                    {
                        fn traverse_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> Vec<&'__a98shdai mut Self::To> {
                            self.0.traverse_mut(&mut source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::PrismMut<#to>,
                        #data_gen_where
                    {
                        fn pm_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> Option<&'__a98shdai mut Self::To> {
                            self.0.pm_mut(&mut source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::LensMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::LensMut<#to>,
                        #data_gen_where
                    {
                        fn view_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> &'__a98shdai mut Self::To {
                            self.0.view_mut(&mut source.#field_name)
                        }
                    }

                };

                let impl_mv = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::Traversal<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::Traversal<#to>,
                        #data_gen_where
                    {
                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> where Self::To: Sized {
                            self.0.traverse(source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::Prism<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::Prism<#to>,
                        #data_gen_where
                    {
                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> where Self::To: Sized {
                            self.0.pm(source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::Lens<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::Lens<#to>,
                        #data_gen_where
                    {
                        fn view(&self, source: #data_name #data_gen) -> Self::To where Self::To: Sized {
                            self.0.view(source.#field_name)
                        }
                    }
                };

                match mutability {
                    OpticMutability::Ref(_) => vec![impl_ref],
                    OpticMutability::Mut(_) => vec![impl_mut, impl_ref],
                    OpticMutability::Move   => vec![impl_mv, impl_mut, impl_ref]
                }.into_iter().flat_map(|x| x)
            }).collect(),
        Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(fs), .. }) => fs
            .unnamed
            .iter()
            .take(7)
            .filter(|var| {
                var
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
            })
            .enumerate()
            .flat_map(|(i, f)| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.iter().collect::<Vec<_>>();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let optics_name = format_ident!("_{}", i);
                let to = &f.ty;
                let field_name = syn::Index::from(i);

                let attr: syn::Attribute = f
                    .attrs
                    .clone()
                    .into_iter()
                    .find(|attr: &syn::Attribute| attr.path.is_ident(&syn::Ident::new("optic", Span::call_site())))
                    .unwrap();
                let mutability = syn::parse::<OpticMutability>(TokenStream::from(attr.tokens)).unwrap();

                let impl_ref = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalRef<#to>,
                        #data_gen_where
                    {
                        type To = __Tr::To;

                        fn traverse_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Vec<&'__a98shdai Self::To> {
                            self.0.traverse_ref(&source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::PrismRef<#to>,
                        #data_gen_where
                    {
                        fn pm_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> Option<&'__a98shdai Self::To> {
                            self.0.pm_ref(&source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::LensRef<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::LensRef<#to>,
                        #data_gen_where
                    {
                        fn view_ref<'__a98shdai>(&self, source: &'__a98shdai #data_name #data_gen) -> &'__a98shdai Self::To {
                            self.0.view_ref(&source.#field_name)
                        }
                    }
                };

                let impl_mut = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::TraversalMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::TraversalMut<#to>,
                        #data_gen_where
                    {
                        fn traverse_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Vec<&'a mut Self::To> {
                            self.0.traverse_mut(&mut source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::PrismMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::PrismMut<#to>,
                        #data_gen_where
                    {
                        fn pm_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> Option<&'__a98shdai mut Self::To> {
                            self.0.pm_mut(&mut source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::LensMut<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::LensMut<#to>,
                        #data_gen_where
                    {
                        fn view_mut<'__a98shdai>(&self, source: &'__a98shdai mut #data_name #data_gen) -> &'__a98shdai mut Self::To {
                            self.0.view_mut(&mut source.#field_name)
                        }
                    }

                };

                let impl_mv = quote! {
                    impl<#(#data_gen_param,)* __Tr> lens_rs::Traversal<#data_name #data_gen> for lens_rs::optics::#optics_name<__Tr>
                    where
                        __Tr: lens_rs::Traversal<#to>,
                        #data_gen_where
                    {
                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> where Self::To: Sized {
                            self.0.traverse(source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Pm> lens_rs::Prism<#data_name #data_gen> for lens_rs::optics::#optics_name<__Pm>
                    where
                        __Pm: lens_rs::Prism<#to>,
                        #data_gen_where
                    {
                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> where Self::To: Sized {
                            self.0.pm(source.#field_name)
                        }
                    }

                    impl<#(#data_gen_param,)* __Ls> lens_rs::Lens<#data_name #data_gen> for lens_rs::optics::#optics_name<__Ls>
                    where
                        __Ls: lens_rs::Lens<#to>,
                        #data_gen_where
                    {
                        fn view(&self, source: #data_name #data_gen) -> Self::To where Self::To: Sized {
                            self.0.view(source.#field_name)
                        }
                    }
                };

                match mutability {
                    OpticMutability::Ref(_) => vec![impl_ref],
                    OpticMutability::Mut(_) => vec![impl_mut, impl_ref],
                    OpticMutability::Move   => vec![impl_mv, impl_mut, impl_ref]
                }.into_iter().flat_map(|x| x)
            }).collect(),
        _ => panic!("union and enum can't derive the lens"),
    };

    TokenStream::from(lens)
}

struct OpticCollector<'a>(&'a mut OpticMap);

impl<'a> OpticCollector<'a> {
    fn collect_optic_fields<'f>(&mut self, fields: impl Iterator<Item = &'f syn::Field>) {
        fields.for_each(|field| {
            if field.attrs.iter().any(|attr| {
                attr.path
                    .is_ident(&syn::Ident::new("optic", Span::call_site()))
            }) {
                field
                    .ident
                    .as_ref()
                    .map(|ident| format!("{}", ident.to_string()))
                    .map(|optic_name| self.0.insert(optic_name));
            }
        });
    }
}

impl<'a> Visit<'_> for OpticCollector<'a> {
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

    fn visit_item_enum(&mut self, item_enum: &ItemEnum) {
        item_enum.variants.iter().for_each(|variant| {
            if variant.attrs.iter().any(|attr| {
                attr.path
                    .is_ident(&syn::Ident::new("optic", Span::call_site()))
            }) {
                self.0.insert(format!("{}", variant.ident));
            }
        })
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
        let optic_ident = syn::Ident::new(&optic_name, Span::call_site());
        struct_items.push(parse_quote! {

               #[derive(Copy, Clone, Debug, Eq, PartialEq)]
               #[allow(non_camel_case_types)]
               pub struct #optic_ident<Optic>(pub Optic);
        });
    }

    quote!( #( #struct_items )* ).into()
}
