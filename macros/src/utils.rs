use syn::{Attribute, Ident, ItemFn, MetaList, PatIdent, PatType, ReturnType, Type};
extern crate quote;
use quote::{__private::TokenStream, quote};

pub fn function_params_from_itemfn(function: &mut ItemFn) -> Vec<(Ident, Vec<Attribute>, String)> {
    let mut result = Vec::new();
    for param in &mut function.sig.inputs {
        match param {
            syn::FnArg::Receiver(_) => panic!(),
            syn::FnArg::Typed(PatType { pat, ty, attrs, .. }) => match *pat.clone() {
                syn::Pat::Ident(PatIdent { ident, .. }) => {
                    result.push((ident, attrs.clone(), quote!(#ty).to_string()));
                    *attrs = Vec::new();
                }
                _ => panic!(),
            },
        }
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
    params.iter().fold(quote!(), |acc, param| {
        let import = arg_import_from_function_param(param);
        quote!(
            #acc
            #import
        )
    })
}

fn arg_import_from_function_param(
    (ident, _attrs, param_type): &(Ident, Vec<Attribute>, String),
) -> TokenStream {
    let ident_str = ident.to_string();
    let get_variable = quote!(interpreter.get_variable(#ident_str).unwrap());
    if param_type == "i64" {
        quote!(
            let simplesl::variable::Variable::Int(#ident) = *#get_variable else {
                panic!()
            };
        )
    } else if param_type == "f64" {
        quote!(
            let simplesl::variable::Variable::Float(#ident) = *#get_variable else {
                panic!()
            };
        )
    } else if param_type == "Rc < str >" {
        quote!(
            let simplesl::variable::Variable::String(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        )
    } else if param_type == "& str" {
        quote!(
            let simplesl::variable::Variable::String(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        )
    } else if param_type == "Rc < [Variable] >" {
        quote!(
            let simplesl::variable::Variable::Array(#ident, _) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        )
    } else if param_type == "& [Variable]" {
        quote!(
            let simplesl::variable::Variable::Array(#ident, _) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        )
    } else if param_type == "Rc < Function >" {
        quote!(
            let simplesl::variable::Variable::Function(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        )
    } else if param_type == "& Function" {
        quote!(
            let simplesl::variable::Variable::Function(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        )
    } else if param_type == "Variable" {
        quote!(
            let #ident = #get_variable.clone();
        )
    } else if param_type == "& Variable" {
        quote!(
            let #ident = #get_variable;
        )
    } else if param_type == "& mut Interpreter" {
        quote!()
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

pub fn params_from_function_params(params: &[(Ident, Vec<Attribute>, String)]) -> TokenStream {
    params.iter().fold(quote!(), |acc, param| {
        if param.2 == "& mut Interpreter" {
            quote!()
        } else {
            let param = param_from_function_param(param);
            quote!(#acc #param,)
        }
    })
}

fn param_from_function_param(
    (ident, attrs, param_type): &(Ident, Vec<Attribute>, String),
) -> TokenStream {
    let ident = ident.to_string();
    let param_type = type_from_str(attrs, param_type);
    quote!(
        simplesl::function::Param {
            name: #ident.into(),
            var_type: #param_type,
        }
    )
}

fn type_from_str(attrs: &[Attribute], param_type: &str) -> TokenStream {
    if param_type == "i64" {
        quote!(simplesl::variable::Type::Int)
    } else if param_type == "f64" {
        quote!(simplesl::variable::Type::Float)
    } else if param_type == "Rc < str >" || param_type == "& str" {
        quote!(simplesl::variable::Type::String)
    } else if param_type == "Rc < [Variable] >" || param_type == "& [Variable]" {
        get_type_from_attrs(attrs).unwrap_or(quote!(simplesl::variable::Type::Array(
            simplesl::variable::Type::Any.into()
        )))
    } else if param_type == "Rc < Function >" || param_type == "& Function" {
        let Some(var_type) = get_type_from_attrs(attrs) else {
            panic!("Argument of type function must be precede by var_type attribute")
        };
        var_type
    } else if param_type == "Variable" || param_type == "& Variable" {
        get_type_from_attrs(attrs).unwrap_or(quote!(simplesl::variable::Type::Any))
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

fn get_type_from_attrs(attrs: &[Attribute]) -> Option<TokenStream> {
    for attr in attrs {
        match &attr.meta {
            syn::Meta::List(MetaList { path, tokens, .. })
                if quote!(#path).to_string() == "var_type" =>
            {
                return Some(quote!(
                    {use std::str::FromStr; simplesl::variable::Type::from_str(#tokens).unwrap()}
                ))
            }
            _ => (),
        };
    }
    None
}

fn return_type_from_syn_type(return_type: &Type) -> TokenStream {
    let return_type = quote!(#return_type).to_string();
    if return_type == "i64"
        || return_type == "Result < i64 >"
        || return_type == "bool"
        || return_type == "Result < bool >"
        || return_type == "usize"
        || return_type == "Result < usize >"
    {
        quote!(simplesl::variable::Type::Int)
    } else if return_type == "f64" || return_type == "Result < f64 >" {
        quote!(simplesl::variable::Type::Float)
    } else if return_type == "Rc < str >"
        || return_type == "Result < Rc < str > >"
        || return_type == "String"
        || return_type == "Result < String >"
        || return_type == "& str"
        || return_type == "Result < & str >"
    {
        quote!(simplesl::variable::Type::String)
    } else if return_type == "Rc < [Variable] >" || return_type == "Result < Rc < [Variable] > >" {
        quote!(simplesl::variable::Type::Array(
            simplesl::variable::Type::Any.into()
        ))
    } else if return_type.is_empty() {
        quote!(simplesl::variable::Type::Void)
    } else if return_type == "Variable" || return_type == "Result < Variable >" {
        quote!(simplesl::variable::Type::Any)
    } else if return_type == "io :: Result < String >"
        || return_type == "std :: io :: Result < String >"
    {
        quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("string|(int,string)").unwrap()
        })
    } else if return_type == "io :: Result < () >" || return_type == "std :: io :: Result < () >" {
        quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("()|(int,string)").unwrap()
        })
    } else if return_type == "Option < i64 >" {
        quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("int|()").unwrap()
        })
    } else if return_type == "Option < f64 >" {
        quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("float|()").unwrap()
        })
    } else {
        panic!("{return_type} type isn't allowed")
    }
}

fn is_result(return_type: &Type) -> bool {
    let return_type = quote!(#return_type).to_string();
    return_type.starts_with("Result")
}

pub fn get_body(is_result: bool, ident: &Ident, args: &TokenStream) -> TokenStream {
    if is_result {
        quote!(Ok(#ident(#args)?.into()))
    } else {
        quote!(Ok(#ident(#args).into()))
    }
}

pub fn get_return_type(function: &ItemFn, return_type: Option<TokenStream>) -> (TokenStream, bool) {
    let ReturnType::Type(_, syn_type) = &function.sig.output else {
        return (quote!(simplesl::variable::Type::Void), false);
    };
    let return_type = return_type.unwrap_or_else(|| return_type_from_syn_type(syn_type));
    (return_type, is_result(syn_type))
}
