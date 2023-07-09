use crate::{
    function::{NativeFunction, Param, Params},
    interpreter::VariableMap,
    variable::Variable,
    variable_type::Type,
};
use simplesl_macros::export_function;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn int_to_float(value: i64) -> f64 {
        value as f64
    }

    #[export_function]
    fn float_to_int(value: f64) -> i64 {
        value as i64
    }
}
