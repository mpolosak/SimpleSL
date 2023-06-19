extern crate proc_macro;
use proc_macro::TokenStream;
extern crate quote;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};
mod utils;
use utils::{
    args_from_function_params, args_import_from_function_params, function_params_from_itemfn,
    get_body, params_from_function_params, return_type_from_str, return_type_to_str,
};

#[proc_macro_attribute]
pub fn export_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let ident = function.sig.ident.clone();
    let ident_str = if attr.is_empty() {
        ident.to_string()
    } else {
        parse_macro_input!(attr as LitStr).value()
    };
    let params = function_params_from_itemfn(function.clone());
    let args_importing = args_import_from_function_params(&params);
    let args = args_from_function_params(&params);
    let params = params_from_function_params(&params);
    let fnreturn_type = return_type_to_str(&function);
    let return_type = return_type_from_str(&fnreturn_type);
    let body = get_body(&fnreturn_type, ident, args);
    quote!(
        #function
        variables.add_native_function(
            #ident_str,
            NativeFunction {
                params: Params {
                    standard: params!(#params),
                    catch_rest: None,
                },
                return_type: #return_type,
                body: |_name, intepreter, args| {
                    #args_importing
                    #body
                },
            },
        );
    )
    .into()
}
