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
    let param_type = type_from_rust_type(attrs, param_type);
    quote!(
        simplesl::function::Param {
            name: #ident.into(),
            var_type: #param_type,
        }
    )
}

fn type_from_rust_type(attrs: &[Attribute], param_type: &Box<Type>) -> TokenStream {
    if let Some(var_type) = get_type_from_attrs(attrs) {
        return var_type;
    }
    quote!(<#param_type as simplesl::variable::TypeOf>::type_of())
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

fn is_result(return_type: &Type) -> bool {
    let return_type = quote!(#return_type).to_string();
    return_type.starts_with("Result")
}

pub fn get_body(is_result: bool, ident: &Ident, args: &TokenStream) -> TokenStream {
    if is_result {
        return quote!(Ok(#ident(#args)?.into()));
    }
    quote!(Ok(#ident(#args).into()))
}

pub fn get_return_type(function: &ItemFn, return_type: Option<TokenStream>) -> (TokenStream, bool) {
    let ReturnType::Type(_, syn_type) = &function.sig.output else {
        return (type_from_str("()"), false);
    };
    let return_type =
        return_type.unwrap_or_else(|| quote!(<#syn_type as simplesl::variable::TypeOf>::type_of()));
    (return_type, is_result(syn_type))
}
