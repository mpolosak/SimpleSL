mod array;
mod iofunctions;
mod math;
mod string;
mod types;
use crate::{
    error::Error,
    function::{NativeFunction, Param, Params},
    interpreter::Interpreter,
    variable::{Type, Variable},
};
use simplesl_macros::export_function;

use crate::interpreter::VariableMap;

pub fn add_std_lib(variables: &mut VariableMap) {
    array::add_functions(variables);
    iofunctions::add_functions(variables);
    types::add_functions(variables);
    math::add_functions(variables);
    string::add_functions(variables);

    #[export_function]
    fn import(interpreter: &mut Interpreter, path: &str) -> Result<Variable, Error> {
        interpreter.load_and_exec(path)
    }
}
