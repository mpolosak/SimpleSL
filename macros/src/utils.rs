use syn::{Ident, ItemFn, PatIdent, PatType};
extern crate quote;
use quote::quote;

pub fn param_idents_from_function(function: ItemFn) -> Vec<Ident> {
    function
        .sig
        .inputs
        .into_iter()
        .map(|param| match param {
            syn::FnArg::Receiver(_) => panic!(),
            syn::FnArg::Typed(PatType { pat, .. }) => match *pat {
                syn::Pat::Ident(PatIdent { ident, .. }) => ident,
                _ => panic!(),
            },
        })
        .collect()
}

pub fn args_from_idents(idents: &[Ident]) -> quote::__private::TokenStream {
    idents
        .iter()
        .fold(quote!(), |acc, ident| quote!(#acc #ident,))
}

pub fn args_importing_from_idents(idents: &[Ident]) -> quote::__private::TokenStream {
    idents.iter().fold(quote!(), |acc, ident| {
        let ident_str = ident.to_string();
        quote!(
            #acc
            let Variable::Float(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    })
}

pub fn params_from_idents(idents: &[Ident]) -> quote::__private::TokenStream {
    let params = idents
        .iter()
        .take(idents.len() - 1)
        .fold(quote!(), |acc, ident| {
            let ident = ident.to_string();
            quote!(#acc #ident: Type::Float,)
        });
    let last = idents.last().unwrap().to_string();
    quote!(#params #last: Type::Float)
}
