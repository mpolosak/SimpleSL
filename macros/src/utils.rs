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
    match param_type.as_str() {
        "i64" => quote!(
            let simplesl::variable::Variable::Int(#ident) = *#get_variable else {
                panic!()
            };
        ),
        "f64" => quote!(
            let simplesl::variable::Variable::Float(#ident) = *#get_variable else {
                panic!()
            };
        ),
        "Arc < str >" => quote!(
            let simplesl::variable::Variable::String(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        ),
        "& str" => quote!(
            let simplesl::variable::Variable::String(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        ),
        "Arc < [Variable] >" => quote!(
            let simplesl::variable::Variable::Array(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        ),
        "& [Variable]" => quote!(
            let simplesl::variable::Variable::Array(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        ),
        "Arc < Function >" => quote!(
            let simplesl::variable::Variable::Function(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.clone();
        ),
        "& Function" => quote!(
            let simplesl::variable::Variable::Function(#ident) = #get_variable else {
                panic!()
            };
            let #ident = #ident.as_ref();
        ),
        "Variable" => quote!(
            let #ident = #get_variable.clone();
        ),
        "& Variable" => quote!(
            let #ident = #get_variable;
        ),
        "& mut Interpreter" => quote!(),
        param_type => panic!("{param_type} type isn't allowed"),
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
    match param_type {
        "i64" => quote!(simplesl::variable::Type::Int),
        "f64" => quote!(simplesl::variable::Type::Float),
        "Arc < str >" | "& str" => quote!(simplesl::variable::Type::String),
        "Arc < [Variable] >" | "& [Variable]" => {
            get_type_from_attrs(attrs).unwrap_or(quote!([simplesl::variable::Type::Any].into()))
        }
        "Arc < Function >" | "& Function" => {
            let Some(var_type) = get_type_from_attrs(attrs) else {
                panic!("Argument of type function must be precede by var_type attribute")
            };
            var_type
        }
        "Variable" | "& Variable" => {
            get_type_from_attrs(attrs).unwrap_or(quote!(simplesl::variable::Type::Any))
        }
        param_type => panic!("{param_type} type isn't allowed"),
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
    match quote!(#return_type).to_string().as_str() {
        "i64"
        | "Result < i64, ExecError >"
        | "bool"
        | "Result < bool, ExecError >"
        | "usize"
        | "Result < usize, ExecError >" => {
            quote!(simplesl::variable::Type::Int)
        }
        "f64" | "Result < f64, ExecError >" => quote!(simplesl::variable::Type::Float),
        "Arc < str >"
        | "Result < Arc < str >, ExecError >"
        | "String"
        | "Result < String, ExecError >"
        | "& str"
        | "Result < & str, ExecError >" => quote!(simplesl::variable::Type::String),
        "Arc < [Variable] >" | "Result < Arc < [Variable], ExecError > >" => {
            quote!([simplesl::variable::Type::Any].into())
        }
        "" => quote!(simplesl::variable::Type::Void),
        "Variable" | "Result < Variable, ExecError >" => quote!(simplesl::variable::Type::Any),
        "io :: Result < String >" | "std :: io :: Result < String >" => quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("string|(int,string)").unwrap()
        }),
        "io :: Result < () >" | "std :: io :: Result < () >" => quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("()|(int,string)").unwrap()
        }),
        "Option < i64 >" => quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("int|()").unwrap()
        }),
        "Option < f64 >" => quote!({
            use std::str::FromStr;
            simplesl::variable::Type::from_str("float|()").unwrap()
        }),
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
        return (quote!(simplesl::variable::Type::Void), false);
    };
    let return_type = return_type.unwrap_or_else(|| return_type_from_syn_type(syn_type));
    (return_type, is_result(syn_type))
}
