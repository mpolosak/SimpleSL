extern crate proc_macro;
use proc_macro::TokenStream;
extern crate quote;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};
mod utils;
use utils::{
    args_from_idents, args_importing_from_idents, param_idents_from_function, params_from_idents,
};

#[proc_macro_attribute]
pub fn export_math_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let ident = function.sig.ident.clone();
    let ident_str = if attr.is_empty() {
        ident.to_string()
    } else {
        parse_macro_input!(attr as LitStr).value()
    };
    let params = param_idents_from_function(function.clone());
    let args_importing = args_importing_from_idents(&params);
    let args = args_from_idents(&params);
    let params = params_from_idents(&params);
    quote!(
        #function
        variables.add_native_function(
            #ident_str,
            NativeFunction {
                params: Params {
                    standard: params!(#params),
                    catch_rest: None,
                },
                return_type: Type::Float,
                body: |_name, _intepreter, args| {
                    #args_importing
                    Ok(Variable::Float(#ident(#args)))
                },
            },
        );
    )
    .into()
}
