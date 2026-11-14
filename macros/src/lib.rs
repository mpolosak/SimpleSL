#![warn(clippy::pedantic)]
mod attributes;
mod decl;
mod export;
mod var;
mod var_type;
use crate::{decl::lazy_decl, export::export_module, var::var_token_from_pair};
use decl::decl;
use export::export_item_fn;
use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;
use simplesl_parser::{Rule, SimpleSLParser};
use syn::{Ident, ItemFn, ItemMod, parse, parse_macro_input};
use var::quote;
use var_type::type_quote;

#[proc_macro]
pub fn decls(item: TokenStream) -> TokenStream {
    let str = item.to_string();
    SimpleSLParser::parse(Rule::decls, &str)
        .unwrap_or_else(|error| panic!("{error}"))
        .map(decl)
        .fold(quote!(), |acc, curr| {
            quote!(
                #acc
                #curr
            )
        })
        .into()
}

/// Macro simplifying exporting functions and modules into `SimpleSL`
#[proc_macro_attribute]
pub fn export(attr: TokenStream, module: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as Ident);
    let (module, val) = if let Ok(mut function) = parse::<ItemFn>(module.clone()) {
        let export = export_item_fn(&mut function, None);
        (quote!(#function), export)
    } else {
        let mut module = parse_macro_input!(module as ItemMod);
        let items = export_module(&mut module);
        (quote!(#module), items)
    };

    let decl = lazy_decl(&ident, &val);
    quote!(
        #module
        #decl
    )
    .into()
}

/// Macro simplifying creating `SimpleSL` Type
#[proc_macro]
pub fn var_type(item: TokenStream) -> TokenStream {
    type_quote(&item).into()
}

/// Macro simplifying creating `SimpleSL` Variable
#[proc_macro]
pub fn var(item: TokenStream) -> TokenStream {
    quote(&item).into()
}
