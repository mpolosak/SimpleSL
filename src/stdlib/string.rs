use crate as simplesl;
use crate::{interpreter::Interpreter, variable::Variable};
use match_any::match_any;
use simplesl_macros::export_function;
use std::sync::Arc;

/// Add string part of standard library to Interpreter
pub fn add_string(interpreter: &mut Interpreter) {
    #[export_function(return_type = "[string]")]
    fn split(string: &str, pat: &str) -> Arc<[Variable]> {
        string
            .split(pat)
            .map(|slice| Variable::String(Arc::from(slice)))
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
    fn chars(string: &str) -> Arc<[Variable]> {
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
        match_any! { variable,
            Variable::Array(var) | Variable::String(var) => var.len(),
            _ => unreachable!()
        }
    }
}
