use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display, PartialEq)]
#[display("Type parsing failed")]
pub struct ParseTypeError;
impl Error for ParseTypeError {}
