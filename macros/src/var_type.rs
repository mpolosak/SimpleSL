use pest::{iterators::Pair, Parser};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use simplesl_parser::{unexpected, Rule, SimpleSLParser};

pub fn type_quote(item: &TokenStream) -> TokenStream2 {
    let item_str = item.to_string();
    type_from_str(&item_str)
}

pub fn type_from_str(item_str: &str) -> TokenStream2 {
    let pair = SimpleSLParser::parse(Rule::r#type_ident, item_str)
        .unwrap_or_else(|error| panic!("{error}"))
        .next()
        .unwrap();
    type_token_from_pair(pair)
}

fn type_token_from_pair(pair: Pair<Rule>) -> TokenStream2 {
    match pair.as_rule() {
        Rule::bool_type => quote!(simplesl::variable::Type::Bool),
        Rule::int_type => quote!(simplesl::variable::Type::Int),
        Rule::float_type => quote!(simplesl::variable::Type::Float),
        Rule::string_type => quote!(simplesl::variable::Type::String),
        Rule::void => quote!(simplesl::variable::Type::Void),
        Rule::any => quote!(simplesl::variable::Type::Any),
        Rule::never => quote!(simplesl::variable::Type::Never),
        Rule::multi_ident => pair
            .into_inner()
            .map(|pair| type_token_from_pair(pair))
            .reduce(|acc, curr| quote!(#acc | # curr))
            .unwrap(),
        Rule::array_type_ident => {
            let element_type = pair.into_inner().next().map_or_else(
                || quote!(simplesl::variable::Type::Never),
                type_token_from_pair,
            );
            quote!(simplesl::variable::Type::Array((#element_type).into()))
        }
        Rule::tuple_type_ident => {
            let elements = pair
                .into_inner()
                .map(type_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Type::Tuple([#elements].into()))
        }
        Rule::function_type_ident => {
            let mut pairs = pair.into_inner();
            let params = type_token_from_pair(pairs.next().unwrap());
            let return_type = pairs.next().map(type_token_from_pair).unwrap();
            quote!(simplesl::variable::Type::Function(
                simplesl::variable::FunctionType {
                    params: #params,
                    return_type: #return_type
                }.into()
            ))
        }
        Rule::ident => {
            let ident = format_ident!("{}", pair.as_str());
            quote!(#ident)
        }
        Rule::function_type_params_ident => {
            let elements = pair
                .into_inner()
                .map(type_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!([#elements].into())
        }
        rule => unexpected!(rule),
    }
}
