#![warn(clippy::pedantic)]
mod attributes;
mod export_function;
mod var;
mod var_type;
use attributes::Attributes;
use export_function::{
    args_from_function_params, args_import_from_function_params, function_params_from_itemfn,
    get_return_type, params_from_function_params,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use var::var_quote;
use var_type::type_quote;

/// Macro simplifying exporting Rust function into SimpleSL
#[proc_macro_attribute]
pub fn export_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let attr = Attributes::parse(attr);
    let mut function = parse_macro_input!(function as ItemFn);
    let ident = function.sig.ident.clone();
    let ident_str = attr.name.unwrap_or_else(|| ident.to_string().into());
    let params = function_params_from_itemfn(&mut function);
    let args = args_from_function_params(&params);
    let args_importing = args_import_from_function_params(&params);
    let params = params_from_function_params(&params);
    let return_type = get_return_type(&function, attr.return_type);
    quote!(
        #function
        {
            interpreter.insert(
                #ident_str.into(),
                simplesl::function::Function::new(
                    simplesl::function::Params(std::sync::Arc::new([#params])),
                    |interpreter| {
                        #args_importing
                        simplesl::ToResult::<_, simplesl::errors::ExecError>::to_result(
                            #ident(#args)
                        ).map(|value| value.into())
                    },
                    #return_type,
                ).into(),
            );
        }
    )
    .into()
}

/// Macro simplifying creating SimpleSL Type
#[proc_macro]
pub fn var_type(item: TokenStream) -> TokenStream {
    type_quote(item).into()
}

/// Macro simplifying creating SimpleSL Variable
#[proc_macro]
pub fn var(item: TokenStream) -> TokenStream {
    var_quote(item).into()
}
