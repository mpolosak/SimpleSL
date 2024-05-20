use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct ParseTypeError;
impl Display for ParseTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type parsing failed")
    }
}
impl Error for ParseTypeError {}
