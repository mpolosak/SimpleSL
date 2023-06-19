use std::rc::Rc;

use crate::error::Error;
use crate::function::{Function, NativeFunction, Param, Params};
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

    #[export_function(name = "if")]
    fn function_if(
        intepreter: &mut Intepreter,
        condition: i64,
        #[function(
            return_type: Type::Any.into(),
            params: vec![],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
    ) -> Result<Variable, Error> {
        if condition == 0 {
            Ok(Variable::Null)
        } else {
            function.exec("function", intepreter, Array::new())
        }
    }

    #[export_function]
    fn if_else(
        intepreter: &mut Intepreter,
        condition: i64,
        #[function(
            return_type: Type::Any.into(),
            params: vec![],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
        #[function(
            return_type: Type::Any.into(),
            params: vec![],
            catch_rest: false
        )]
        else_function: Rc<dyn Function>,
    ) -> Result<Variable, Error> {
        if condition == 0 {
            else_function.exec("else", intepreter, Array::new())
        } else {
            function.exec("function", intepreter, Array::new())
        }
    }
}
