use crate::{
    error::Error,
    function::{NativeFunction, Param, Params},
    interpreter::VariableMap,
    variable::{Array, Variable},
    variable_type::Type,
};
use simplesl_macros::export_function;
use std::rc::Rc;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn string_len(string: &str) -> usize {
        string.len()
    }

    #[export_function]
    fn string_at(string: &str, index: i64) -> Result<&str, Error> {
        if index < 0 {
            Err(Error::CannotBeNegative(String::from("index")))
        } else if index as usize > string.len() {
            Err(Error::IndexToBig)
        } else {
            let index = index as usize;
            Ok(string.get(index..index).unwrap())
        }
    }

    #[export_function]
    fn split(string: &str, pat: &str) -> Array {
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

    #[export_function]
    fn chars(string: &str) -> Array {
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
}
