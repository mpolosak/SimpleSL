use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct TypeParsingError;
impl Display for TypeParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type parsing failed")
    }
}
impl Error for TypeParsingError {}
