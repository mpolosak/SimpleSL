#![warn(clippy::pedantic)]
mod attributes;
mod export_function;
mod var;
mod var_type;
use attributes::Attributes;
use export_function::export_item_fn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemConst, ItemFn, ItemMod};
use var::var_quote;
use var_type::type_quote;

/// Macro simplifying exporting modules into SimpleSL
#[proc_macro_attribute]
pub fn export(_attr: TokenStream, module: TokenStream) -> TokenStream {
    let module = parse_macro_input!(module as ItemMod);
    let items = module.content.unwrap().1;
    let items = items
        .into_iter()
        .map(|item| match item {
            Item::Const(ItemConst { ident, expr, .. }) => {
                let ident = ident.to_string();
                quote!(interpreter.insert(#ident.into(), #expr.into());)
            }
            Item::Fn(mut function) => {
                let attributes = Attributes::from_function_attrs(&function.attrs);
                function.attrs = vec![];
                export_item_fn(function, attributes)
            }
            _ => quote!(#item),
        })
        .fold(quote!(), |acc, curr| {
            quote!(
                #acc
                #curr
            )
        });
    let ident = module.ident;
    quote!(
        pub fn #ident(interpreter: &mut simplesl::Interpreter){
            #items
        }
    )
    .into()
}

/// Macro simplifying exporting Rust function into SimpleSL
#[proc_macro_attribute]
pub fn export_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let attr = Attributes::parse(attr);
    let function = parse_macro_input!(function as ItemFn);
    export_item_fn(function, attr).into()
}

/// Macro simplifying creating SimpleSL Type
#[proc_macro]
pub fn var_type(item: TokenStream) -> TokenStream {
    type_quote(item).into()
}

/// Macro simplifying creating SimpleSL Variable
#[proc_macro]
pub fn var(item: TokenStream) -> TokenStream {
    var_quote(item).into()
}
