extern crate proc_macro;
use proc_macro::TokenStream;
extern crate quote;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn export_function_of_two_floats(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let ident = function.sig.ident.clone();
    let ident_str = function.sig.ident.to_string();
    quote!(
        #function
        variables.add_native_function(
            #ident_str,
            NativeFunction {
                params: Params {
                    standard: params!("a": Type::Float, "b": Type::Float),
                    catch_rest: None,
                },
                return_type: Type::Float,
                body: |_name, _intepreter, args| {
                    let Variable::Float(a) = args.get("a")? else {
                        panic!()
                    };
                    let Variable::Float(b) = args.get("b")? else {
                        panic!()
                    };
                    Ok(Variable::Float(#ident(a,b)))
                },
            },
        );
    )
    .into()
}
