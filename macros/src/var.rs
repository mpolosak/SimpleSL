use pest::{iterators::Pair, Parser};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use simplesl_parser::{unexpected, Rule, SimpleSLParser};

use crate::var_type::type_from_str;

pub fn var_quote(item: TokenStream) -> quote::__private::TokenStream {
    let item_str = item.to_string();
    var_from_str(&item_str)
}

pub fn var_from_str(item_str: &str) -> quote::__private::TokenStream {
    let pair = SimpleSLParser::parse(Rule::var_macro, item_str)
        .unwrap_or_else(|error| panic!("{error}"))
        .next()
        .unwrap();
    var_token_from_pair(pair)
}

fn var_token_from_pair(pair: Pair<Rule>) -> quote::__private::TokenStream {
    match pair.as_rule() {
        Rule::int | Rule::minus_int => quote_int(pair),
        Rule::float | Rule::minus_float => {
            let value = pair
                .as_str()
                .replace([' ', '_'], "")
                .parse::<f64>()
                .unwrap();
            quote!(simplesl::variable::Variable::Float(#value))
        }
        Rule::string => {
            let value = pair.into_inner().next().unwrap().as_str();
            let value = unescaper::unescape(value).unwrap();
            quote!(simplesl::variable::Variable::String(#value.into()))
        }
        Rule::void => quote!(simplesl::variable::Variable::Void),
        Rule::array_ident => {
            let element_type = pair
                .clone()
                .into_inner()
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| {
                    let acc = acc?;
                    let curr = curr?;
                    Some(quote!(#acc | # curr))
                })
                .unwrap_or(Some(quote!(simplesl::variable::Type::Never)));
            let elements = pair
                .into_inner()
                .map(var_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            if let Some(element_type) = element_type {
                quote!(simplesl::variable::Variable::Array(
                    simplesl::variable::Array::new_with_type(
                        #element_type,
                        [#elements].into()
                    ).into()
                ))
            } else {
                quote!(simplesl::variable::Variable::from([#elements]))
            }
        }
        Rule::tuple_ident => {
            let elements = pair
                .into_inner()
                .map(var_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Variable::Tuple([#elements].into()))
        }
        Rule::array_ident_repeat => {
            let mut inner = pair.into_inner();
            let value_pair = inner.next().unwrap();
            let value = var_token_from_pair(value_pair.clone());
            let element_type = var_type_from_var_pair(value_pair);
            let len_pair = inner.next().unwrap();
            let len = if len_pair.as_rule() == Rule::ident {
                let ident = format_ident!("{}", len_pair.as_str());
                quote!(#ident as usize)
            } else {
                let value = parse_int(inner.next().unwrap()) as usize;
                quote!(#value)
            };
            if let Some(element_type) = element_type {
                quote!(simplesl::variable::Variable::Array(
                    simplesl::variable::Array::new_with_type(
                        #element_type,
                        std::iter::repeat(#value).take(#len).collect()
                    ).into()
                ))
            } else {
                quote!(simplesl::variable::Variable::Array(
                    simplesl::variable::Array::new_repeat(#value, #len).into()
                ))
            }
        }
        Rule::ident => {
            let ident = format_ident!("{}", pair.as_str());
            quote!(simplesl::variable::Variable::from(#ident))
        }
        Rule::minus_ident => {
            let ident = format_ident!("{}", pair.as_str());
            quote!(simplesl::variable::Variable::from(-#ident))
        }
        rule => unexpected(rule),
    }
}

fn parse_int(pair: Pair<Rule>) -> i64 {
    match pair.as_rule() {
        Rule::int => parse_int(pair.into_inner().next().unwrap()),
        Rule::minus_int => -parse_int(pair.into_inner().next().unwrap()),
        Rule::binary_int => parse_int_with_radix(pair, 2),
        Rule::octal_int => parse_int_with_radix(pair, 8),
        Rule::decimal_int => parse_int_with_radix(pair, 10),
        Rule::hexadecimal_int => parse_int_with_radix(pair, 16),
        rule => unexpected(rule),
    }
}

fn parse_int_with_radix(pair: Pair<Rule>, radix: u32) -> i64 {
    let inner = pair
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .replace([' ', '_'], "");
    i64::from_str_radix(&inner, radix).unwrap()
}

fn quote_int(pair: Pair<Rule>) -> quote::__private::TokenStream {
    let value = parse_int(pair);
    quote!(simplesl::variable::Variable::Int(#value))
}

fn var_type_from_var_pair(pair: Pair<Rule>) -> Option<quote::__private::TokenStream> {
    match pair.as_rule() {
        Rule::int | Rule::minus_int => Some(type_from_str("int")),
        Rule::float | Rule::minus_float => Some(type_from_str("float")),
        Rule::string => Some(type_from_str("string")),
        Rule::void => Some(type_from_str("()")),
        Rule::array_ident => {
            let element_type = pair
                .into_inner()
                .map(|pair| pair.into_inner().next().unwrap())
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| {
                    let acc = acc?;
                    let curr = curr?;
                    Some(quote!(#acc | # curr))
                })
                .unwrap_or(Some(quote!(simplesl::variable::Type::Never)))?;
            Some(quote!(simplesl::variable::Type::Array(#element_type.into())))
        }
        Rule::tuple_ident => {
            let elements = pair
                .into_inner()
                .map(|pair| pair.into_inner().next().unwrap())
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| {
                    let acc = acc?;
                    let curr = curr?;
                    Some(quote!(#acc, # curr))
                })
                .unwrap()?;
            Some(quote!(simplesl::variable::Type::Tuple([#elements].into())))
        }
        Rule::array_ident_repeat => {
            let element_type = var_type_from_var_pair(
                pair.into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap(),
            )?;
            Some(quote!(simplesl::variable::Type::Array(#element_type.into())))
        }
        Rule::ident | Rule::minus_ident => None,
        rule => unexpected(rule),
    }
}
