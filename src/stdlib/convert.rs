use crate as simplesl;
use crate::{interpreter::Interpreter, variable::Variable};
use simplesl_macros::export_function;

/// Add type convertion part of standard library to Interpreter
pub fn add_convert(interpreter: &mut Interpreter) {
    #[export_function]
    fn to_float(#[var_type("int|float")] value: &Variable) -> f64 {
        match value {
            Variable::Int(value) => *value as f64,
            Variable::Float(value) => *value,
            _ => unreachable!("value required to be int | float"),
        }
    }

    #[export_function]
    fn float_to_int(value: f64) -> i64 {
        value as i64
    }

    #[export_function]
    fn parse_int(string: &str) -> Option<i64> {
        string.parse::<i64>().ok()
    }

    #[export_function]
    fn parse_float(string: &str) -> Option<f64> {
        string.parse::<f64>().ok()
    }

    #[export_function]
    fn to_string(variable: Variable) -> String {
        variable.to_string()
    }
}
