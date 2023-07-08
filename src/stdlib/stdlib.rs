use crate::error::Error;
use crate::function::{NativeFunction, Param, Params};
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
}
