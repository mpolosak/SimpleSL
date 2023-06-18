use crate::function::{NativeFunction, Param, Params};
use crate::intepreter::VariableMap;
use crate::{params, variable::Variable, variable_type::Type};
use simplesl_macros::export_math_function;

pub fn add_math_functions(variables: &mut VariableMap) {
    #[export_math_function]
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[export_math_function]
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[export_math_function]
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[export_math_function]
    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }

    #[export_math_function]
    fn modulo(a: f64, b: f64) -> f64 {
        let divided = a / b;
        a - b * divided.floor()
    }

    #[export_math_function]
    fn or(a: f64, b: f64) -> f64 {
        a.abs() + b.abs()
    }

    #[export_math_function]
    fn not(a: f64) -> f64 {
        if a == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}
