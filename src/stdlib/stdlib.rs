use std::rc::Rc;

use crate::error::Error;
use crate::function::{Function, NativeFunction, Param, Params};
use crate::interpreter::Interpreter;
use crate::variable_type::Type;
use crate::{interpreter::VariableMap, variable::*};
use simplesl_macros::export_function;
extern crate simplesl_macros;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn import(interpreter: &mut Interpreter, path: &str) -> Result<Variable, Error> {
        interpreter.load_and_exec(path)
    }

    #[export_function]
    fn equals(a: Variable, b: Variable) -> bool {
        a == b
    }

    #[export_function(name = "if")]
    fn function_if(
        interpreter: &mut Interpreter,
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
            function.exec("function", interpreter, Array::new())
        }
    }

    #[export_function]
    fn if_else(
        interpreter: &mut Interpreter,
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
            else_function.exec("else", interpreter, Array::new())
        } else {
            function.exec("function", interpreter, Array::new())
        }
    }
}
