use crate::error::Error;
use crate::function::{NativeFunction, Param, Params};
use crate::interpreter::VariableMap;
use crate::variable::{Array, Variable};
use crate::variable_type::Type;
use simplesl_macros::export_function;
use std::rc::Rc;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function]
    fn string_concat(string1: &str, string2: &str) -> Rc<str> {
        format!("{string1}{string2}").into()
    }

    #[export_function]
    fn string_len(string: &str) -> i64 {
        string.len() as i64
    }

    #[export_function]
    fn string_at(string: &str, index: i64) -> Result<Rc<str>, Error> {
        if index < 0 {
            Err(Error::CannotBeNegative(String::from("index")))
        } else if index as usize > string.len() {
            Err(Error::IndexToBig)
        } else {
            let index = index as usize;
            Ok(string.get(index..index).unwrap().into())
        }
    }

    #[export_function]
    fn split(string: &str, pat: &str) -> Rc<Array> {
        string
            .split(pat)
            .map(|slice| Variable::String(Rc::from(slice)))
            .collect::<Array>()
            .into()
    }

    #[export_function]
    fn replace(string: &str, from: &str, to: &str) -> Rc<str> {
        string.replace(from, to.as_ref()).into()
    }

    #[export_function]
    fn string_contains(string: &str, pat: &str) -> i64 {
        string.contains(pat).into()
    }

    #[export_function]
    fn chars(string: &str) -> Rc<Array> {
        string
            .chars()
            .map(|char| Variable::String(char.to_string().into()))
            .collect::<Array>()
            .into()
    }

    #[export_function]
    fn to_lowercase(string: &str) -> Rc<str> {
        string.to_lowercase().into()
    }

    #[export_function]
    fn to_uppercase(string: &str) -> Rc<str> {
        string.to_uppercase().into()
    }
}
