extern crate proc_macro;
use proc_macro::TokenStream;
extern crate quote;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
mod attributes;
mod utils;
use attributes::Attributes;
use utils::{
    args_from_function_params, args_import_from_function_params, function_params_from_itemfn,
    get_body, params_from_function_params, return_type_from_str, return_type_to_str,
};

#[proc_macro_attribute]
pub fn export_function(attr: TokenStream, function: TokenStream) -> TokenStream {
    let attr = Attributes::parse(attr);
    let function = parse_macro_input!(function as ItemFn);
    let ident = function.sig.ident.clone();
    let ident_str = if let Some(value) = attr.name {
        value
    } else {
        ident.to_string()
    };
    let mut params = function_params_from_itemfn(function.clone());
    let args = args_from_function_params(&params);
    let args_importing = args_import_from_function_params(&params);
    let catch_rest = if attr.catch_rest {
        match params.pop() {
            Some((ident, type_str)) if type_str == "Rc < Array >" => {
                let ident = ident.to_string();
                quote!(Some(String::from(#ident)))
            }
            Some(_) | None => {
                panic!("catch_rest=true requiers function to have last param of type Rc<Array>")
            }
        }
    } else {
        quote!(None)
    };
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
                    catch_rest: #catch_rest,
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
