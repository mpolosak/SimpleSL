#![warn(clippy::pedantic)]
mod attributes;
mod export_function;
mod var;
mod var_type;
use attributes::Attributes;
use export_function::export_item_fn;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemConst, ItemFn, ItemMod, ItemUse, Visibility, parse_macro_input};
use var::quote;
use var_type::type_quote;

/// Macro simplifying exporting modules into `SimpleSL`
#[proc_macro_attribute]
pub fn export(_attr: TokenStream, module: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(module as ItemMod);
    let Some(items) = &mut module.content else {
        panic!("Cannot export module from another file")
    };
    let mod_ident = &module.ident;
    let items = items
        .1
        .iter_mut()
        .map(|item| match item {
            Item::Const(ItemConst {
                ident,
                vis: Visibility::Public(_) | Visibility::Restricted(_),
                ..
            }) => {
                let ident_string = ident.to_string();
                quote!(interpreter.insert(#ident_string.into(), (#mod_ident::#ident).into());)
            }
            Item::Fn(function) => {
                if matches!(function.vis, Visibility::Inherited) {
                    return quote!();
                }
                let attributes = Attributes::from_function_attrs(&function.attrs);
                function.attrs = vec![];
                export_item_fn(function, attributes, Some(&module.ident))
            }
            Item::Use(
                item @ ItemUse {
                    vis: Visibility::Public(_) | Visibility::Restricted(_),
                    ..
                },
            ) => {
                quote!(#item)
            }
            _ => quote!(),
        })
        .fold(quote!(), |acc, curr| {
            quote!(
                #acc
                #curr
            )
        });
    quote!(
        #module
        pub fn #mod_ident(interpreter: &mut simplesl::Interpreter){
            #items
        }
    )
    .into()
}

/// Macro simplifying exporting Rust function into `SimpleSL`
#[proc_macro_attribute]
pub fn export_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let attr = Attributes::parse(attr);
    let mut function = parse_macro_input!(function as ItemFn);
    let export = export_item_fn(&mut function, attr, None);
    quote!(
        #function
        #export
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
