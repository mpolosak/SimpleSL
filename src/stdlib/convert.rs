use crate::{
    function,
    interpreter::Interpreter,
    variable::{Type, Variable},
};
use simplesl_macros::export_function;

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn int_to_float(value: i64) -> f64 {
        value as f64
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
