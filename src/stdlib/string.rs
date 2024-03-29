use crate as simplesl;
use crate::{interpreter::Interpreter, variable::Variable};
use simplesl_macros::export_function;
use std::rc::Rc;

/// Add string part of standard library to Interpreter
pub fn add_string(interpreter: &mut Interpreter) {
    #[export_function(return_type = "[string]")]
    fn split(string: &str, pat: &str) -> Rc<[Variable]> {
        string
            .split(pat)
            .map(|slice| Variable::String(Rc::from(slice)))
            .collect()
    }

    #[export_function]
    fn replace(string: &str, from: &str, to: &str) -> String {
        string.replace(from, to.as_ref())
    }

    #[export_function]
    fn string_contains(string: &str, pat: &str) -> bool {
        string.contains(pat)
    }

    #[export_function(return_type = "[string]")]
    fn chars(string: &str) -> Rc<[Variable]> {
        string
            .chars()
            .map(|char| Variable::String(char.to_string().into()))
            .collect()
    }

    #[export_function]
    fn to_lowercase(string: &str) -> String {
        string.to_lowercase()
    }

    #[export_function]
    fn to_uppercase(string: &str) -> String {
        string.to_uppercase()
    }

    #[export_function]
    fn len(#[var_type("[any]|string")] variable: Variable) -> usize {
        match variable {
            Variable::Array(array) => array.len(),
            Variable::String(string) => string.len(),
            _ => unreachable!(),
        }
    }
}
