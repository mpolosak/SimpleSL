use pest::{iterators::Pair, Parser};
use proc_macro::TokenStream;
use quote::quote;
use simplesl_parser::{unexpected, Rule, SimpleSLParser};

pub fn type_quote(item: TokenStream) -> quote::__private::TokenStream {
    let item_str = item.to_string();
    type_from_str(&item_str)
}

pub fn type_from_str(item_str: &str) -> quote::__private::TokenStream {
    let pair = SimpleSLParser::parse(Rule::r#type, item_str)
        .unwrap_or_else(|error| panic!("{error}"))
        .next()
        .unwrap();
    type_token_from_pair(pair)
}

fn type_token_from_pair(pair: Pair<Rule>) -> quote::__private::TokenStream {
    match pair.as_rule() {
        Rule::int_type => quote!(simplesl::variable::Type::Int),
        Rule::float_type => quote!(simplesl::variable::Type::Float),
        Rule::string_type => quote!(simplesl::variable::Type::String),
        Rule::void => quote!(simplesl::variable::Type::Void),
        Rule::any => quote!(simplesl::variable::Type::Any),
        Rule::multi => pair
            .into_inner()
            .map(|pair| type_token_from_pair(pair))
            .reduce(|acc, curr| quote!(#acc | # curr))
            .unwrap(),
        Rule::array_type => {
            let element_type = pair.into_inner().next().map(type_token_from_pair).unwrap();
            quote!(simplesl::variable::Type::Array((#element_type).into()))
        }
        Rule::tuple_type => {
            let elements = pair
                .into_inner()
                .map(type_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            quote!(simplesl::variable::Type::Tuple([#elements].into()))
        }
        Rule::function_type => {
            let mut pairs = pair.into_inner();
            let params = pairs
                .next()
                .unwrap()
                .into_inner()
                .map(type_token_from_pair)
                .reduce(|acc, curr| quote!(#acc, # curr));
            let return_type = pairs.next().map(type_token_from_pair).unwrap();
            quote!(simplesl::variable::Type::Function(
                simplesl::variable::FunctionType {
                    params: [#params].into(),
                    return_type: #return_type
                }.into()
            ))
        }
        rule => unexpected(rule),
    }
}
