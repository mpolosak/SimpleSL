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

pub fn add_std_lib(interpreter: &mut Interpreter) {
    array::add_functions(interpreter);
    iofunctions::add_functions(interpreter);
    types::add_functions(interpreter);
    math::add_functions(interpreter);
    string::add_functions(interpreter);

    #[export_function]
    fn import(interpreter: &mut Interpreter, path: &str) -> Result<Variable, Error> {
        interpreter.remove_layer();
        let result = interpreter.load_and_exec(path);
        interpreter.add_layer();
        result
    }
    #[export_function]
    fn len(#[var_type("[any]|string")] variable: Variable) -> usize {
        match variable {
            Variable::Array(array, _) => array.len(),
            Variable::String(string) => string.len(),
            _ => panic!(),
        }
    }
}
