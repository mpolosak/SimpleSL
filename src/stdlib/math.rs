use crate::function::{NativeFunction, Param, Params};
use crate::interpreter::VariableMap;
use crate::{variable::Variable, variable_type::Type};
use simplesl_macros::export_function;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    #[export_function]
    fn subtract(a: i64, b: i64) -> i64 {
        a - b
    }

    #[export_function]
    fn multiply(a: i64, b: i64) -> i64 {
        a * b
    }

    #[export_function]
    fn divide(a: i64, b: i64) -> i64 {
        a / b
    }

    #[export_function]
    fn modulo(a: i64, b: i64) -> i64 {
        a % b
    }

    #[export_function]
    fn addf(a: f64, b: f64) -> f64 {
        a + b
    }

    #[export_function]
    fn subtractf(a: f64, b: f64) -> f64 {
        a - b
    }

    #[export_function]
    fn multiplyf(a: f64, b: f64) -> f64 {
        a * b
    }

    #[export_function]
    fn dividef(a: f64, b: f64) -> f64 {
        a / b
    }

    #[export_function]
    fn int_to_float(value: i64) -> f64 {
        value as f64
    }

    #[export_function]
    fn float_to_int(value: f64) -> i64 {
        value as i64
    }

    #[export_function]
    fn bin_and(a: i64, b: i64) -> i64 {
        a & b
    }

    #[export_function]
    fn bin_or(a: i64, b: i64) -> i64 {
        a | b
    }

    #[export_function]
    fn xor(a: i64, b: i64) -> i64 {
        a ^ b
    }

    #[export_function]
    fn shift(a: i64, bits: i64) -> i64 {
        if bits > 0 {
            a << bits
        } else {
            a >> -bits
        }
    }
}
