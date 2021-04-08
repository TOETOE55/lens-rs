use inwelling::*;

use proc_macro2::Span;
use std::collections::HashSet;
use std::{env, fs, path::PathBuf};
use syn::visit::Visit;
use syn::{ItemEnum, ItemStruct};

fn main() {
    let mut optics_set = OpticsSet::new();
    let mut optics_collector = OpticsCollector(&mut optics_set);

    for section in inwelling(Opts {
        watch_manifest: true,
        watch_rs_files: true,
        dump_rs_paths: true,
    })
    .sections
    {
        for rs_path in section.rs_paths.unwrap() {
            let contents = String::from_utf8(fs::read(rs_path).unwrap()).unwrap();
            let syntax = syn::parse_file(&contents)
                .expect(".rs files should contain valid Rust source code.");
            optics_collector.visit_file(&syntax);
        }
    }

    let mut output = String::new();
    for optic_name in optics_set {
        if let "Some" | "None" | "Ok" | "Err" = &*optic_name {
            continue;
        }
        output += &format!(
            r"

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct {}<Optics>(pub Optics);

        ",
            optic_name
        );
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("$OUT_DIR should exist."));
    std::fs::write(out_path.join("optics.rs"), output).expect("optics.rs should be generated.");
}

type OpticsSet = HashSet<String>;

struct OpticsCollector<'a>(&'a mut OpticsSet);

impl<'a> Visit<'_> for OpticsCollector<'a> {
    fn visit_item_enum(&mut self, item_enum: &ItemEnum) {
        for variant in &item_enum.variants {
            if variant_with_optic_attr(variant) {
                self.0.insert(format!("{}", variant.ident));
            }
        }
    }

    fn visit_item_struct(&mut self, item_struct: &ItemStruct) {
        if let syn::Fields::Named(fields_named) = &item_struct.fields {
            for field in &fields_named.named {
                if field_with_optic_attr(field) {
                    self.0.insert(format!("{}", field.ident.clone().unwrap()));
                }
            }
        }
    }
}

fn variant_with_optic_attr(var: &syn::Variant) -> bool {
    var.attrs.iter().any(|attr| {
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
