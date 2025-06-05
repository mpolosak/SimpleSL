use crate as simplesl;
use simplesl_macros::export;

#[export]
pub mod add_convert {
    pub use crate::variable::Variable;

    pub fn to_float(#[var_type(int|float)] value: &Variable) -> f64 {
        match value {
            Variable::Int(value) => *value as f64,
            Variable::Float(value) => *value,
            _ => unreachable!("value required to be int | float"),
        }
    }

    pub fn to_int(#[var_type(int|float)] value: &Variable) -> i64 {
        match value {
            Variable::Int(value) => *value,
            Variable::Float(value) => *value as i64,
            _ => unreachable!("value required to be int | float"),
        }
    }

    pub fn parse_int(string: &str) -> Option<i64> {
        string.parse::<i64>().ok()
    }

    pub fn parse_float(string: &str) -> Option<f64> {
        string.parse::<f64>().ok()
    }

    pub fn to_string(variable: &Variable) -> String {
        variable.to_string()
    }
}
