use crate::var_type::{type_from_str, type_quote};
use quote::{__private::TokenStream, quote};
use syn::{Attribute, Ident, ItemFn, MetaList, PatIdent, PatType, ReturnType, Type};

pub fn function_params_from_itemfn(function: &mut ItemFn) -> Vec<(Ident, Vec<Attribute>, String)> {
    let mut result = Vec::new();
    for param in &mut function.sig.inputs {
        let syn::FnArg::Typed(PatType { pat, ty, attrs, .. }) = param else {
            panic!()
        };
        let syn::Pat::Ident(PatIdent { ident, .. }) = *pat.clone() else {
            panic!()
        };
        result.push((ident, attrs.clone(), quote!(#ty).to_string()));
        *attrs = Vec::new();
    }
    result
}

pub fn args_from_function_params(
    params: &[(Ident, Vec<Attribute>, String)],
) -> quote::__private::TokenStream {
    params
        .iter()
        .fold(quote!(), |acc, (ident, ..)| quote!(#acc #ident,))
}

pub fn args_import_from_function_params(params: &[(Ident, Vec<Attribute>, String)]) -> TokenStream {
    params.iter().fold(quote!(), |acc, (ident, _, _)| {
        let ident_str = ident.to_string();
        quote!(
            #acc
            let #ident = interpreter.get_variable(#ident_str).unwrap().try_into().unwrap();
        )
    })
}

pub fn params_from_function_params(params: &[(Ident, Vec<Attribute>, String)]) -> TokenStream {
    params.iter().fold(quote!(), |acc, param| {
        let param = param_from_function_param(param);
        quote!(#acc #param,)
    })
}

fn param_from_function_param(
    (ident, attrs, param_type): &(Ident, Vec<Attribute>, String),
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

fn type_from_rust_type(attrs: &[Attribute], param_type: &str) -> TokenStream {
    match param_type {
        "i64" => type_from_str("int"),
        "f64" => type_from_str("f64"),
        "Arc < str >" | "& str" => type_from_str("string"),
        "Arc < [Variable] >" | "& [Variable]" => {
            get_type_from_attrs(attrs).unwrap_or(type_from_str("[any]"))
        }
        "Arc < Function >" | "& Function" => {
            let Some(var_type) = get_type_from_attrs(attrs) else {
                panic!("Argument of type function must be precede by var_type attribute")
            };
            var_type
        }
        "Variable" | "& Variable" => get_type_from_attrs(attrs).unwrap_or(type_from_str("any")),
        param_type => panic!("{param_type} type isn't allowed"),
    }
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

fn return_type_from_syn_type(return_type: &Type) -> TokenStream {
    match quote!(#return_type).to_string().as_str() {
        "i64"
        | "Result < i64, ExecError >"
        | "bool"
        | "Result < bool, ExecError >"
        | "usize"
        | "Result < usize, ExecError >" => type_from_str("int"),
        "f64" | "Result < f64, ExecError >" => type_from_str("float"),
        "Arc < str >"
        | "Result < Arc < str >, ExecError >"
        | "String"
        | "Result < String, ExecError >"
        | "& str"
        | "Result < & str, ExecError >" => type_from_str("string"),
        "Arc < [Variable] >" | "Result < Arc < [Variable], ExecError > >" => type_from_str("[any]"),
        "" => type_from_str("()"),
        "Variable" | "Result < Variable, ExecError >" => type_from_str("any"),
        "io :: Result < String >" | "std :: io :: Result < String >" => {
            type_from_str("string|(int,string)")
        }
        "io :: Result < () >" | "std :: io :: Result < () >" => type_from_str("()|(int,string)"),
        "Option < i64 >" => type_from_str("int|()"),
        "Option < f64 >" => type_from_str("float|()"),
        return_type => panic!("{return_type} type isn't allowed"),
    }
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
    let return_type = return_type.unwrap_or_else(|| return_type_from_syn_type(syn_type));
    (return_type, is_result(syn_type))
}
