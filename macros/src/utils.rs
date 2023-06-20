use syn::{Attribute, Ident, ItemFn, MetaList, PatIdent, PatType, ReturnType};
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
    } else if param_type == "Rc < dyn Function >" {
        quote!(
            let Variable::Function(#ident) = args.get(#ident_str)? else {
                panic!()
            };
        )
    } else if param_type == "Variable" {
        quote!(
            let #ident = args.get(#ident_str)?;
        )
    } else if param_type == "& mut Interpreter" {
        quote!()
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

pub fn params_from_function_params(params: &[(Ident, Vec<Attribute>, String)]) -> TokenStream {
    params.iter().fold(quote!(), |acc, param| {
        if param.2 != "& mut Interpreter" {
            let param = param_from_function_param(param);
            quote!(#acc #param,)
        } else {
            quote!()
        }
    })
}

fn param_from_function_param(
    (ident, attrs, param_type): &(Ident, Vec<Attribute>, String),
) -> quote::__private::TokenStream {
    let ident = ident.to_string();
    if param_type == "i64" {
        quote!(
            Param {
                name: String::from(#ident),
                var_type: Type::Int,
            }
        )
    } else if param_type == "f64" {
        quote!(
            Param {
                name: String::from(#ident),
                var_type: Type::Float,
            }
        )
    } else if param_type == "Rc < str >" {
        quote!(
            Param {
                name: String::from(#ident),
                var_type: Type::String,
            }
        )
    } else if param_type == "Rc < Array >" {
        quote!(
            Param {
                name: String::from(#ident),
                var_type: Type::Array,
            }
        )
    } else if param_type == "Rc < dyn Function >" {
        if attrs.is_empty() {
            panic!("Argument of type function must be precede by function attribute")
        }
        for attr in attrs {
            match &attr.meta {
                syn::Meta::List(MetaList { path, tokens, .. })
                    if quote!(#path).to_string() == "function" =>
                {
                    return quote!(
                        Param {
                            name: String::from(#ident),
                            var_type: Type::Function{
                                #tokens
                            },
                        }
                    )
                }
                _ => (),
            };
        }
        panic!("Argument of type function must be precede by function attribute")
    } else if param_type == "Variable" {
        quote!(
            Param {
                name: String::from(#ident),
                var_type: Type::Any,
            }
        )
    } else {
        panic!("{param_type} type isn't allowed")
    }
}

pub fn return_type_from_str(return_type: &str) -> TokenStream {
    if return_type == "i64" || return_type == "Result < i64, Error >" {
        quote!(Type::Int)
    } else if return_type == "f64" || return_type == "Result < f64, Error >" {
        quote!(Type::Float)
    } else if return_type == "Rc < str >" || return_type == "Result < Rc < str >, Error >" {
        quote!(Type::String)
    } else if return_type == "Rc < Array >" || return_type == "Result < Rc < Array >, Error >" {
        quote!(Type::Array)
    } else if return_type.is_empty() {
        quote!(Type::Null)
    } else if return_type == "Variable" || return_type == "Result < Variable, Error >" {
        quote!(Type::Any)
    } else {
        panic!("{return_type} type isn't allowed")
    }
}

pub fn get_body(return_type: &str, ident: Ident, args: TokenStream) -> TokenStream {
    let return_type = return_type.to_string();
    if return_type == "i64"
        || return_type == "f64"
        || return_type == "Rc < str >"
        || return_type == "Rc < Array >"
    {
        quote!(Ok(#ident(#args).into()))
    } else if return_type == "Variable" {
        quote!(Ok(#ident(#args)))
    } else if return_type.is_empty() {
        quote!(
            #ident(#args);
            Ok(Variable::Null)
        )
    } else if return_type == "Result < Variable, Error >"
        || return_type == "Result < i64, Error >"
        || return_type == "Result < f64, Error >"
        || return_type == "Result < Rc < str >, Error >"
        || return_type == "Result < Rc < Array >, Error >"
    {
        quote!(Ok(#ident(#args)?.into()))
    } else {
        panic!()
    }
}

pub fn return_type_to_str(function: &ItemFn) -> String {
    if let ReturnType::Type(_, return_type) = &function.sig.output {
        quote!(#return_type).to_string()
    } else {
        String::from("")
    }
}
