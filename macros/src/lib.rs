#![warn(clippy::pedantic)]
mod attributes;
mod export;
mod var;
mod var_type;
use export::export_item_fn;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, ItemFn, ItemMod, parse, parse_macro_input};
use var::quote;
use var_type::type_quote;

use crate::export::export_module;

/// Macro simplifying exporting functions and modules into `SimpleSL`
#[proc_macro_attribute]
pub fn export(attr: TokenStream, module: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as Ident);
    let (module, mod_ident, val) = if let Ok(mut function) = parse::<ItemFn>(module.clone()) {
        let export = export_item_fn(&mut function, None);
        (quote!(#function), function.sig.ident.clone(), export)
    } else {
        let mut module = parse_macro_input!(module as ItemMod);
        let items = export_module(&mut module);
        (quote!(#module), module.ident, items)
    };

    let var_ident = format_ident!("{}_var", mod_ident);
    quote!(
        #module
        lazy_static::lazy_static! {
            static ref #var_ident: simplesl::variable::Variable = {
                #val
            };
        }
        pub struct #ident;

        impl From<#ident> for simplesl::variable::Variable{
            fn from(_: #ident) -> simplesl::variable::Variable {
                #var_ident.clone()
            }
        }
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
