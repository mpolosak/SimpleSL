use pest::{iterators::Pair, Parser};
use proc_macro::TokenStream;
use quote::quote;
use simplesl_parser::{unexpected, Rule, SimpleSLParser};

use crate::var_type::type_from_str;

pub fn var_quote(item: TokenStream) -> quote::__private::TokenStream {
    let item_str = item.to_string();
    var_from_str(&item_str)
}

pub fn var_from_str(item_str: &str) -> quote::__private::TokenStream {
    let pair = SimpleSLParser::parse(Rule::only_var, item_str)
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
        Rule::array_from_str => {
            let element_type = pair
                .clone()
                .into_inner()
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| quote!(#acc | # curr));
            let elements = pair
                .into_inner()
                .map(var_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Variable::Array(
                simplesl::variable::Array::new_with_type(
                    #element_type,
                    [#elements].into()
                ).into()
            ))
        }
        Rule::tuple_from_str => {
            let elements = pair
                .into_inner()
                .map(var_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Variable::Tuple([#elements].into()))
        }
        Rule::array_repeat_from_str => {
            let mut inner = pair.into_inner();
            let value_pair = inner.next().unwrap();
            let value = var_token_from_pair(value_pair.clone());
            let element_type = var_type_from_var_pair(value_pair);
            let len = parse_int(inner.next().unwrap().into_inner().next().unwrap()) as usize;
            quote!(simplesl::variable::Variable::Array(
                simplesl::variable::Array::new_with_type(
                    #element_type,
                    std::iter::repeat(#value).take(#len).collect()
                ).into()
            ))
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

fn var_type_from_var_pair(pair: Pair<Rule>) -> quote::__private::TokenStream {
    match pair.as_rule() {
        Rule::int | Rule::minus_int => type_from_str("int"),
        Rule::float | Rule::minus_float => type_from_str("float"),
        Rule::string => type_from_str("string"),
        Rule::void => type_from_str("()"),
        Rule::array_from_str => {
            let element_type = pair
                .into_inner()
                .map(|pair| pair.into_inner().next().unwrap())
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| quote!(#acc | # curr));
            quote!([#element_type])
        }
        Rule::tuple_from_str => {
            let elements = pair
                .into_inner()
                .map(|pair| pair.into_inner().next().unwrap())
                .map(var_type_from_var_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Type::Tuple([#elements].into()))
        }
        Rule::array_repeat_from_str => {
            let element_type = var_type_from_var_pair(
                pair.into_inner()
                    .next()
                    .unwrap()
                    .into_inner()
                    .next()
                    .unwrap(),
            );
            quote!(simplesl::variable::Type::Array(#element_type.into()))
        }
        rule => unexpected(rule),
    }
}
