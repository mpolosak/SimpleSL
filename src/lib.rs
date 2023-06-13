pub mod error;
pub mod function;
pub mod intepreter;
mod parse;
pub mod stdlib;
pub mod variable;
pub mod variable_type;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::fmt::{Debug, Display};

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
