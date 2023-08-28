use crate as simplesl;
mod convert;
mod fs;
mod iofunctions;
mod string;
use crate::{interpreter::Interpreter, variable::Variable};
use simplesl_macros::export_function;

pub fn add_std_lib(interpreter: &mut Interpreter) {
    iofunctions::add_functions(interpreter);
    convert::add_functions(interpreter);
    string::add_functions(interpreter);
    fs::add_functions(interpreter);

    #[export_function]
    fn len(#[var_type("[any]|string")] variable: Variable) -> usize {
        match variable {
            Variable::Array(array, _) => array.len(),
            Variable::String(string) => string.len(),
            _ => panic!(),
        }
    }
}
