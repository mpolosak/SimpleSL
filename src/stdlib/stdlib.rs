use std::rc::Rc;

use crate::error::Error;
use crate::function::{NativeFunction, Param, Params};
use crate::intepreter::Intepreter;
use crate::variable_type::Type;
use crate::{intepreter::VariableMap, params, variable::*};
use simplesl_macros::export_function;
extern crate simplesl_macros;

pub fn add_std_functions(variables: &mut VariableMap) {
    #[export_function]
    fn import(intepreter: &mut Intepreter, path: Rc<str>) -> Result<Variable, Error> {
        intepreter.load_and_exec(&path)
    }

    #[export_function]
    fn equals(a: Variable, b: Variable) -> i64 {
        if a == b {
            1
        } else {
            0
        }
    }

    variables.add_native_function(
        "if",
        NativeFunction {
            params: Params {
                standard: params!(
                    "condition": Type::Int,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Int(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                if condition == 0 {
                    Ok(Variable::Null)
                } else {
                    function.exec("function", intepreter, Array::new())
                }
            },
        },
    );
    variables.add_native_function(
        "if_else",
        NativeFunction {
            params: Params {
                standard: params!(
                    "condition": Type::Int,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    },
                    "else": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Int(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let Variable::Function(else_function) = args.get("else")? else {
                    panic!()
                };
                if condition == 0 {
                    else_function.exec("else", intepreter, Array::new())
                } else {
                    function.exec("function", intepreter, Array::new())
                }
            },
        },
    );
}
