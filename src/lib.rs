mod code;
mod error;
mod exec_error;
pub mod function;
mod instruction;
mod interpreter;
mod parse;
pub mod stdlib;
mod test_example_scripts;
pub mod variable;
#[macro_use]
extern crate pest_derive;
pub use simplesl_macros::export_function;
use std::fmt::{Debug, Display};
pub use {code::Code, error::Error, exec_error::ExecError, interpreter::Interpreter};

pub fn join(array: &[impl Display], separator: &str) -> String {
    let [elements @ .., last] = array else {
        return "".into();
    };
    let result = elements.iter().fold("".to_owned(), |acc, current| {
        format!("{acc}{current}{separator}")
    });
    format!("{result}{last}")
}

pub fn join_debug(array: &[impl Debug], separator: &str) -> String {
    let [elements @ .., last] = array else {
        return "".into();
    };
    let result = elements.iter().fold("".to_owned(), |acc, current| {
        format!("{acc}{current:?}{separator}")
    });
    format!("{result}{last:?}")
}
