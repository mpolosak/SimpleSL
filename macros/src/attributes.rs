use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::{parse::Parser, punctuated::Punctuated, Expr, ExprLit, Lit, MetaNameValue, Token};

#[derive(Default)]
pub struct Attributes {
    pub name: Option<Rc<str>>,
    pub catch_rest: bool,
    pub return_type: Option<quote::__private::TokenStream>,
}

impl Attributes {
    pub fn parse(attr: TokenStream) -> Attributes {
        let attr = Punctuated::<MetaNameValue, Token![,]>::parse_terminated
            .parse(attr)
            .unwrap();
        let mut new = Attributes::default();
        for MetaNameValue { path, value, .. } in attr {
            let path = quote!(#path).to_string();
            if path == "name" {
                new.name = match value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => Some(lit.value().into()),
                    _ => panic!("{path} must be str literal"),
                }
            } else if path == "return_type" {
                new.return_type = match value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => Some(quote!({use std::str::FromStr; Type::from_str(#lit).unwrap()})),
                    _ => panic!("{path} must be bool"),
                }
            }
        }
        new
    }
}
