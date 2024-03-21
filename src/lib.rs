mod code;
mod errors;
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
pub use {code::Code, errors::Error, errors::ExecError, interpreter::Interpreter};

pub fn join(array: &[impl Display], separator: &str) -> String {
    array
        .iter()
        .map(ToString::to_string)
        .collect::<Box<_>>()
        .join(separator)
}

pub fn join_debug(array: &[impl Debug], separator: &str) -> String {
    array
        .iter()
        .map(|element| format!("{element:?}"))
        .collect::<Box<_>>()
        .join(separator)
}
