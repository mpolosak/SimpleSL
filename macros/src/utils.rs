use syn::{Ident, ItemFn, PatIdent, PatType, ReturnType};
extern crate quote;
use quote::{__private::TokenStream, quote};

pub fn function_params_from_itemfn(function: ItemFn) -> Vec<(Ident, String)> {
    function
        .sig
        .inputs
        .into_iter()
        .map(|param| match param {
            syn::FnArg::Receiver(_) => panic!(),
            syn::FnArg::Typed(PatType { pat, ty, .. }) => match *pat {
                syn::Pat::Ident(PatIdent { ident, .. }) => (ident, quote!(#ty).to_string()),
                _ => panic!(),
            },
        })
        .collect()
}

pub fn args_from_function_params(params: &[(Ident, String)]) -> quote::__private::TokenStream {
    params
        .iter()
        .fold(quote!(), |acc, (ident, _)| quote!(#acc #ident,))
}

pub fn args_import_from_function_params(
    params: &[(Ident, String)],
) -> quote::__private::TokenStream {
    params.iter().fold(quote!(), |acc, param| {
        let import = arg_import_from_function_param(param);
        quote!(
            #acc
            #import
        )
    })
}

fn arg_import_from_function_param(
    (ident, param_type): &(Ident, String),
) -> quote::__private::TokenStream {
    let ident_str = ident.to_string();
    if param_type == "i64" {
        quote!(
            let Variable::Int(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    } else if param_type == "f64" {
        quote!(
            let Variable::Float(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    } else if param_type == "Rc < str >" {
        quote!(
            let Variable::String(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    } else if param_type == "Rc < Array >" {
        quote!(
            let Variable::Array(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    } else if param_type == "Variable" {
        quote!(
            let #ident = args.get(#ident_str)?;
        )
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

pub fn params_from_function_params(fnparams: &[(Ident, String)]) -> TokenStream {
    let params = fnparams
        .iter()
        .take(fnparams.len() - 1)
        .fold(quote!(), |acc, param| {
            let param = param_from_function_param(param);
            quote!(#acc #param,)
        });
    let last = param_from_function_param(fnparams.last().unwrap());
    quote!(#params #last)
}

fn param_from_function_param(
    (ident, param_type): &(Ident, String),
) -> quote::__private::TokenStream {
    let ident = ident.to_string();
    if param_type == "i64" {
        quote!(#ident: Type::Int)
    } else if param_type == "f64" {
        quote!(#ident: Type::Float)
    } else if param_type == "Rc < str >" {
        quote!(#ident: Type::String)
    } else if param_type == "Rc < Array >" {
        quote!(#ident: Type::Array)
    } else if param_type == "Variable" {
        quote!(#ident: Type::Any)
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

pub fn return_type_from_function(function: ItemFn) -> TokenStream {
    let return_type = function.sig.output;
    let return_type = if let ReturnType::Type(_, return_type) = return_type {
        quote!(#return_type).to_string()
    } else {
        String::from("")
    };
    if return_type == "i64" {
        quote!(Type::Int)
    } else if return_type == "f64" {
        quote!(Type::Float)
    } else if return_type == "Rc < str >" {
        quote!(Type::String)
    } else if return_type == "Rc < Array >" {
        quote!(Type::Array)
    } else if return_type.is_empty() {
        quote!(Type::Null)
    } else if return_type == "Variable" {
        quote!(Type::Any)
    } else {
        panic!("{return_type} type isn't allowed")
    }
}

pub fn get_body(return_type: &TokenStream, ident: Ident, args: TokenStream) -> TokenStream {
    let return_type = return_type.to_string();
    if return_type == "Type :: Int" {
        quote!(Ok(Variable::Int(#ident(#args))))
    } else if return_type == "Type :: Float" {
        quote!(Ok(Variable::Float(#ident(#args))))
    } else if return_type == "Type :: String" {
        quote!(Ok(Variable::String(#ident(#args))))
    } else if return_type == "Type :: Array" {
        quote!(Ok(Variable::Array(#ident(#args))))
    } else if return_type == "Type :: Any" {
        quote!(Ok(#ident(#args)))
    } else if return_type == "Type :: Null" {
        quote!(
            #ident(#args);
            Ok(Variable::Null)
        )
    } else {
        panic!()
    }
}
