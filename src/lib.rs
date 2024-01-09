mod code;
mod error;
pub mod function;
mod instruction;
mod interpreter;
mod parse;
pub mod stdlib;
pub mod variable;
#[macro_use]
extern crate pest_derive;
pub use simplesl_macros::export_function;
use std::fmt::{Debug, Display};
pub use {code::Code, error::Error, interpreter::Interpreter};

pub type Result<T> = std::result::Result<T, Error>;

pub fn join(array: &[impl Display], separator: &str) -> String {
    let mut result = String::new();
    if let [elements @ .., last] = array {
        for var in elements {
            result += &format!("{var}{separator}");
        }
        result += &format!("{last}");
    }
    result
}

pub fn join_debug(array: &[impl Debug], separator: &str) -> String {
    let mut result = String::new();
    if let [elements @ .., last] = array {
        for var in elements {
            result += &format!("{var:?}{separator}");
        }
        result += &format!("{last:?}");
    }
    result
}
