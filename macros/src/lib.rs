#![warn(clippy::pedantic)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
mod attributes;
mod utils;
use attributes::Attributes;
use utils::{
    args_from_function_params, args_import_from_function_params, function_params_from_itemfn,
    get_body, get_return_type, params_from_function_params,
};

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
    let (return_type, is_result) = get_return_type(&function, attr.return_type);
    let body = get_body(is_result, &ident, &args);
    quote!(
        #function
        {
            use std::rc::Rc;
            interpreter.insert(
                #ident_str.into(),
                simplesl::function::Function::new(
                    simplesl::function::Params(Rc::new([#params])),
                    |interpreter| {
                        #args_importing
                        #body
                    },
                    #return_type,
                ).into(),
            );
        }
    )
    .into()
}
