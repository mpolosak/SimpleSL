use crate::{
    function::{NativeFunction, Param, Params},
    interpreter::VariableMap,
    variable::{Type, Variable},
};
use simplesl_macros::export_function;
use std::rc::Rc;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn int(value: Variable, fallback: i64) -> i64 {
        if let Variable::Int(value) = value {
            value
        } else {
            fallback
        }
    }

    #[export_function]
    fn float(value: Variable, fallback: f64) -> f64 {
        if let Variable::Float(value) = value {
            value
        } else {
            fallback
        }
    }

    #[export_function]
    fn string(value: Variable, fallback: Rc<str>) -> Rc<str> {
        if let Variable::String(value) = value {
            value
        } else {
            fallback
        }
    }

    #[export_function]
    fn array(value: Variable, fallback: Rc<[Variable]>) -> Rc<[Variable]> {
        if let Variable::Array(value, _) = value {
            value
        } else {
            fallback
        }
    }
}
