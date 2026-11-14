use pest::iterators::Pair;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use simplesl_parser::Rule;

use crate::var_token_from_pair;

pub fn lazy_decl(ident: &Ident, var: &TokenStream) -> TokenStream {
    quote!(
        pub struct #ident;

        impl #ident {
            const simplesl_hidden: std::sync::LazyLock<simplesl::variable::Variable> = std::sync::LazyLock::new(||{#var});
        }

        impl From<#ident> for simplesl::variable::Variable{
            fn from(_: #ident) -> simplesl::variable::Variable {
                #ident::simplesl_hidden.clone()
            }
        }
    )
}

pub fn decl(pair: Pair<'_, Rule>) -> TokenStream {
    let mut inner = pair.into_inner();
    let ident_str = inner.next().unwrap().as_str();
    let ident = format_ident!("{ident_str}");
    let pair = inner.next().unwrap();

    let var = match pair.as_rule() {
        Rule::function => {
            let str = pair.as_str();
            quote!(
                simplesl::Code::parse(&simplesl::Interpreter::without_stdlib(), #str)
                    .unwrap()
                    .exec()
                    .unwrap()
            )
        }
        _ => var_token_from_pair(pair),
    };
    lazy_decl(&ident, &var)
}
