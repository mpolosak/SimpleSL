use crate::var_type::type_from_str;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::rc::Rc;
use syn::{parse::Parser, punctuated::Punctuated, Expr, ExprLit, Lit, MetaNameValue, Token};

#[derive(Default)]
pub struct Attributes {
    pub name: Option<Rc<str>>,
    pub return_type: Option<TokenStream2>,
}

impl Attributes {
    pub fn parse(attr: TokenStream) -> Attributes {
        let attr = Punctuated::<MetaNameValue, Token![,]>::parse_terminated
            .parse(attr)
            .unwrap();
        let mut new = Attributes::default();
        for MetaNameValue { path, value, .. } in attr {
            let path = quote!(#path).to_string();
            match (path.as_ref(), value) {
                (
                    "name",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ) => {
                    new.name = Some(lit.value().into());
                }
                (
                    "return_type",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ) => {
                    new.return_type = Some(type_from_str(&lit.value()));
                }
                ("name" | "return_type", _) => {
                    panic!("{path} must be str literal");
                }
                _ => (),
            }
        }
        new
    }
}
