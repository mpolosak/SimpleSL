use pest::iterators::Pair;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use simplesl_parser::Rule;

use crate::var_token_from_pair;

pub fn lazy_decl(ident: &Ident, ident_hidden: &Ident, var: &TokenStream) -> TokenStream {
    quote!(
        lazy_static::lazy_static! {
            static ref #ident_hidden: simplesl::variable::Variable = {
                #var
            };
        }
        pub struct #ident;

        impl From<#ident> for simplesl::variable::Variable{
            fn from(_: #ident) -> simplesl::variable::Variable {
                #ident_hidden.clone()
            }
        }
    )
}

pub fn decl(pair: Pair<'_, Rule>) -> TokenStream {
    let mut inner = pair.into_inner();
    let ident_str = inner.next().unwrap().as_str();
    let ident = format_ident!("{ident_str}");
    let ident_hidden = format_ident!("{}_var", ident_str.to_lowercase());
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
    lazy_decl(&ident, &ident_hidden, &var)
}
