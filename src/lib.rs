mod bin_operator;
mod code;
mod errors;
pub mod function;
mod instruction;
mod interpreter;
pub mod stdlib;
mod to_result;
mod unary_operator;
pub mod variable;
pub use simplesl_macros::{export_function, var, var_type};
use std::fmt::{Debug, Display};
pub use {
    bin_operator::BinOperator, code::Code, errors::Error, errors::ExecError,
    interpreter::Interpreter, to_result::ToResult,
};

pub fn join<'a, T, I>(items: I, separator: &str) -> String
where
    T: Display + 'a,
    I: IntoIterator<Item = &'a T>,
{
    items
        .into_iter()
        .map(ToString::to_string)
        .collect::<Box<[_]>>()
        .join(separator)
}

pub fn join_debug<'a, T, I>(items: I, separator: &str) -> String
where
    T: Debug + 'a,
    I: IntoIterator<Item = &'a T>,
{
    items
        .into_iter()
        .map(|element| format!("{element:?}"))
        .collect::<Box<[_]>>()
        .join(separator)
}
