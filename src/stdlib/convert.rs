use crate as simplesl;
use crate::variable::Variable;
use simplesl_macros::export;

#[export]
mod add_convert {
    fn to_float(#[var_type(int|float)] value: &Variable) -> f64 {
        match value {
            Variable::Int(value) => *value as f64,
            Variable::Float(value) => *value,
            _ => unreachable!("value required to be int | float"),
        }
    }

    fn to_int(#[var_type(int|float)] value: &Variable) -> i64 {
        match value {
            Variable::Int(value) => *value,
            Variable::Float(value) => *value as i64,
            _ => unreachable!("value required to be int | float"),
        }
    }

    fn parse_int(string: &str) -> Option<i64> {
        string.parse::<i64>().ok()
    }

    fn parse_float(string: &str) -> Option<f64> {
        string.parse::<f64>().ok()
    }

    fn to_string(variable: &Variable) -> String {
        variable.to_string()
    }
}
