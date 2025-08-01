use crate::var_type::type_from_str;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::rc::Rc;
use syn::{Attribute, Expr, ExprLit, Lit, MetaList, MetaNameValue};

#[derive(Default)]
pub struct Attributes {
    pub name: Option<Rc<str>>,
    pub return_type: Option<TokenStream2>,
}

impl Attributes {
    pub fn from_function_attrs(attrs: &Vec<Attribute>) -> Self {
        let mut new = Self::default();
        for Attribute { meta, .. } in attrs {
            match meta {
                syn::Meta::List(MetaList { path, tokens, .. })
                    if quote!(#path).to_string() == "return_type" =>
                {
                    new.return_type = Some(type_from_str(&tokens.to_string()));
                }
                syn::Meta::NameValue(MetaNameValue {
                    path,
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(lit), ..
                        }),
                    ..
                }) if quote!(#path).to_string() == "name" => new.name = Some(lit.value().into()),
                _ => (),
            }
        }
        new
    }
}
