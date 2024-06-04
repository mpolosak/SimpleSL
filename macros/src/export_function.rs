use crate::var_type::{type_from_str, type_quote};
use quote::{__private::TokenStream, quote};
use syn::{Attribute, Ident, ItemFn, MetaList, PatIdent, PatType, ReturnType, Type};

pub fn function_params_from_itemfn(
    function: &mut ItemFn,
) -> Vec<(Ident, Vec<Attribute>, Box<Type>)> {
    let mut result = Vec::new();
    for param in &mut function.sig.inputs {
        let syn::FnArg::Typed(PatType { pat, ty, attrs, .. }) = param else {
            panic!()
        };
        let syn::Pat::Ident(PatIdent { ident, .. }) = *pat.clone() else {
            panic!()
        };
        result.push((ident, attrs.clone(), ty.clone()));
        *attrs = Vec::new();
    }
    result
}

pub fn args_from_function_params(
    params: &[(Ident, Vec<Attribute>, Box<Type>)],
) -> quote::__private::TokenStream {
    params
        .iter()
        .fold(quote!(), |acc, (ident, ..)| quote!(#acc #ident,))
}

pub fn args_import_from_function_params(
    params: &[(Ident, Vec<Attribute>, Box<Type>)],
) -> TokenStream {
    params.iter().fold(quote!(), |acc, (ident, _, _)| {
        let ident_str = ident.to_string();
        quote!(
            #acc
            let #ident = interpreter.get_variable(#ident_str).unwrap().try_into().unwrap();
        )
    })
}

pub fn params_from_function_params(params: &[(Ident, Vec<Attribute>, Box<Type>)]) -> TokenStream {
    params.iter().fold(quote!(), |acc, param| {
        let param = param_from_function_param(param);
        quote!(#acc #param,)
    })
}

fn param_from_function_param(
    (ident, attrs, param_type): &(Ident, Vec<Attribute>, Box<Type>),
) -> TokenStream {
    let ident = ident.to_string();
    let param_type = get_type_from_attrs(attrs)
        .unwrap_or_else(|| quote!(<#param_type as simplesl::variable::TypeOf>::type_of()));
    quote!(
        simplesl::function::Param {
            name: #ident.into(),
            var_type: #param_type,
        }
    )
}

fn get_type_from_attrs(attrs: &[Attribute]) -> Option<TokenStream> {
    for attr in attrs {
        match &attr.meta {
            syn::Meta::List(MetaList { path, tokens, .. })
                if quote!(#path).to_string() == "var_type" =>
            {
                return Some(type_quote(tokens.clone().into()))
            }
            _ => (),
        };
    }
    None
}

pub fn get_return_type(function: &ItemFn, return_type: Option<TokenStream>) -> TokenStream {
    let ReturnType::Type(_, syn_type) = &function.sig.output else {
        return type_from_str("()");
    };
    let return_type =
        return_type.unwrap_or_else(|| quote!(<#syn_type as simplesl::variable::TypeOf>::type_of()));
    return_type
}
