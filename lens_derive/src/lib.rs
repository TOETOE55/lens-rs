extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Optic)]
pub fn derive_optic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let optics = match derive_input.data {
        Data::Enum(e) => e
            .variants
            .iter()
            .flat_map(|x| {
                let optic_name = format_ident!("_{}", x.ident);
                quote! {
                    #[derive(Copy, Clone)]
                    pub struct #optic_name<Optic>(pub Optic);
                }
            })
            .collect(),
        Data::Struct(st) => st
            .fields
            .iter()
            .flat_map(|x| {
                let optic_name = format_ident!("_{}", x.ident.as_ref()?);
                Some(quote! {
                    #[derive(Copy, Clone)]
                    pub struct #optic_name<Optic>(pub Optic);
                })
            })
            .collect(),
        _ => quote! {},
    };
    TokenStream::from(optics)
}

#[proc_macro_derive(Review)]
pub fn derive_review(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let reviews: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .flat_map(|var| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.clone();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let var_name = &var.ident;
                let optic_name = format_ident!("_{}", var.ident);
                let tys = var
                    .fields
                    .iter()
                    .map(|field| field.ty.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let fields = var
                    .fields
                    .iter()
                    .enumerate()
                    .flat_map(|(i, _)| {
                        let i = syn::Index::from(i);
                        quote! { #i }
                    })
                    .collect::<Vec<_>>();

                quote! {
                    impl<Rv, #data_gen_param> lens::Review<#data_name #data_gen> for #optic_name<Rv>
                    where
                        Rv: lens::Review<(#tys,)>,
                        #data_gen_where
                    {
                        type From = Rv::From;

                        fn review(&self, from: Self::From) -> #data_name #data_gen {
                            let tuple = self.0.review(from);
                            <#data_name>::#var_name(#(tuple . #fields,)*)
                        }
                    }
                }
            })
            .collect(),
        _ => panic!("union and struct can't derive the review"),
    };
    TokenStream::from(reviews)
}

#[proc_macro_derive(Prism)]
pub fn derive_prism(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let prisms: proc_macro2::TokenStream  = match derive_input.data.clone() {
        Data::Enum(e) => e
            .variants
            .iter()
            .flat_map(|var| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.clone();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let var_name = &var.ident;
                let optic_name = format_ident!("_{}", var.ident);
                let ty = var
                    .fields
                    .iter()
                    .map(|field| field.ty.clone())
                    .take(1)
                    .collect::<Punctuated<_, Token![,]>>();

                quote! {
                    impl<Tr, #data_gen_param> lens::Traversal<#data_name #data_gen> for #optic_name<Tr>
                    where
                        Tr: lens::Traversal<#ty>,
                        #data_gen_where
                    {
                        type To = Tr::To;

                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.traverse(x),
                                _ => vec![],
                            }
                        }

                        fn traverse_ref<'a>(&self, source: &'a #data_name #data_gen) -> Vec<&'a Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.traverse_ref(x),
                                _ => vec![],
                            }
                        }

                        fn traverse_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Vec<&'a mut Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.traverse_mut(x),
                                _ => vec![],
                            }
                        }
                    }


                    impl<Pm, #data_gen_param> lens::Prism<#data_name #data_gen> for #optic_name<Pm>
                    where
                        Pm: lens::Prism<#ty>,
                        #data_gen_where
                    {
                        type To = Pm::To;

                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.pm(x),
                                _ => None,
                            }
                        }

                        fn pm_ref<'a>(&self, source: &'a #data_name #data_gen) -> Option<&'a Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.pm_ref(x),
                                _ => None,
                            }
                        }

                        fn pm_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Option<&'a mut Self::To> {
                            use #data_name::*;
                            match source {
                                #var_name(x, ..) => self.0.pm_mut(x),
                                _ => None,
                            }
                        }
                    }
                }
            })
            .collect(),
        _ => panic!("union and struct can't derive the review"),
    };

    TokenStream::from(prisms)
}

#[proc_macro_derive(Lens)]
pub fn derive_lens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let lens: proc_macro2::TokenStream = match derive_input.data.clone() {
        Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fs), .. }) => fs
            .named
            .iter()
            .flat_map(|f| {
                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.clone();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let optics_name = format_ident!("_{}", f.ident.as_ref().unwrap());
                let to = &f.ty;
                let field_name = f.ident.as_ref().unwrap();

                quote! {
                    impl<Tr, #data_gen_param> lens::Traversal<#data_name #data_gen> for #optics_name<Tr>
                    where
                        Tr: Traversal<#to>,
                        #data_gen_where
                    {
                        type To = Tr::To;

                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> {
                            self.0.traverse(source.#field_name)
                        }

                        fn traverse_ref<'a>(&self, source: &'a #data_name #data_gen) -> Vec<&'a Self::To> {
                            self.0.traverse_ref(&source.#field_name)
                        }

                        fn traverse_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Vec<&'a mut Self::To> {
                            self.0.traverse_mut(&mut source.#field_name)
                        }
                    }

                    impl<Pm, #data_gen_param> Prism<#data_name #data_gen> for #optics_name<Pm>
                    where
                        Pm: Prism<#to>,
                        #data_gen_where
                    {
                        type To = Pm::To;

                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> {
                            self.0.pm(source.#field_name)
                        }

                        fn pm_ref<'a>(&self, source: &'a #data_name #data_gen) -> Option<&'a Self::To> {
                            self.0.pm_ref(&source.#field_name)
                        }

                        fn pm_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Option<&'a mut Self::To> {
                            self.0.pm_mut(&mut source.#field_name)
                        }
                    }

                    impl<Ls, #data_gen_param> Lens<#data_name #data_gen> for #optics_name<Ls>
                    where
                        Ls: Lens<#to>,
                        #data_gen_where
                    {
                        type To = Ls::To;

                        fn view(&self, source: #data_name #data_gen) -> Self::To {
                            self.0.view(source.#field_name)
                        }

                        fn view_ref<'a>(&self, source: &'a #data_name #data_gen) -> &'a Self::To {
                            self.0.view_ref(&source.#field_name)
                        }

                        fn view_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> &'a mut Self::To {
                            self.0.view_mut(&mut source.#field_name)
                        }
                    }
                }
            }).collect(),
        Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(fs), .. }) => fs
            .unnamed
            .iter()
            .take(7)
            .enumerate()
            .flat_map(|(i, f)| {

                let data = derive_input.clone();
                let data_name = data.ident;
                let data_gen = data.generics;
                let data_gen_param = data_gen.params.clone();
                let data_gen_where = data_gen
                    .where_clause
                    .iter()
                    .flat_map(|x| x.predicates.clone())
                    .collect::<Punctuated<_, Token![,]>>();

                let optics_name = format_ident!("_{}", i);
                let to = &f.ty;
                let i = syn::Index::from(i);

                quote! {
                    impl<Tr, #data_gen_param> lens::Traversal<#data_name #data_gen> for #optics_name<Tr>
                    where
                        Tr: Traversal<#to>,
                        #data_gen_where
                    {
                        type To = Tr::To;

                        fn traverse(&self, source: #data_name #data_gen) -> Vec<Self::To> {
                            self.0.traverse(source.#i)
                        }

                        fn traverse_ref<'a>(&self, source: &'a #data_name #data_gen) -> Vec<&'a Self::To> {
                            self.0.traverse_ref(&source.#i)
                        }

                        fn traverse_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Vec<&'a mut Self::To> {
                            self.0.traverse_mut(&mut source.#i)
                        }
                    }

                    impl<Pm, #data_gen_param> Prism<#data_name #data_gen> for #optics_name<Pm>
                    where
                        Pm: Prism<#to>,
                        #data_gen_where
                    {
                        type To = Pm::To;

                        fn pm(&self, source: #data_name #data_gen) -> Option<Self::To> {
                            self.0.pm(source.#i)
                        }

                        fn pm_ref<'a>(&self, source: &'a #data_name #data_gen) -> Option<&'a Self::To> {
                            self.0.pm_ref(&source.#i)
                        }

                        fn pm_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> Option<&'a mut Self::To> {
                            self.0.pm_mut(&mut source.#i)
                        }
                    }

                    impl<Ls, #data_gen_param> Lens<#data_name #data_gen> for #optics_name<Ls>
                    where
                        Ls: Lens<#to>,
                        #data_gen_where
                    {
                        type To = Ls::To;

                        fn view(&self, source: #data_name #data_gen) -> Self::To {
                            self.0.view(source.#i)
                        }

                        fn view_ref<'a>(&self, source: &'a #data_name #data_gen) -> &'a Self::To {
                            self.0.view_ref(&source.#i)
                        }

                        fn view_mut<'a>(&self, source: &'a mut #data_name #data_gen) -> &'a mut Self::To {
                            self.0.view_mut(&mut source.#i)
                        }
                    }
                }
            }).collect(),
        _ => panic!("union and enum can't derive the lens"),
    };

    TokenStream::from(lens)
}
