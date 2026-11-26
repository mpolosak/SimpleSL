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
use itertools::Itertools;
use pest::Span;
pub use simplesl_macros::{var, var_type};
use std::{
    fmt::{Debug, Display},
    ops::Range,
};
pub use {
    bin_operator::BinOperator, code::Code, errors::Error, errors::ExecError,
    interpreter::Interpreter, to_result::ToResult,
};

pub fn join<'a, T, I>(items: I, separator: &str) -> String
where
    T: Display + 'a,
    I: IntoIterator<Item = &'a T>,
{
    items.into_iter().map(ToString::to_string).join(separator)
}

pub fn join_debug<'a, T, I>(items: I, separator: &str) -> String
where
    T: Debug + 'a,
    I: IntoIterator<Item = &'a T>,
{
    items
        .into_iter()
        .map(|element| format!("{element:?}"))
        .join(separator)
}

pub(crate) fn range_from_pest_span(span: &Span) -> Range<usize> {
    span.start()..span.end()
}

pub(crate) fn range_from_2_ranges(start: &Range<usize>, end: &Range<usize>) -> Range<usize> {
    start.start..end.end
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{Code, Error, ExecError, Interpreter, variable::Variable};

    pub fn parse_and_exec(script: &str) -> Result<Variable, ExecError> {
        Code::parse(&Interpreter::with_stdlib(), script)
            .unwrap()
            .exec()
    }
    pub fn parse_unwrap_first_error(script: &str) -> Error {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap_err()
            .errors
            .swap_remove(0)
            .error
    }

    pub fn parse_unwrap_errors(script: &str) -> Vec<Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap_err()
            .errors
            .into_iter()
            .map(|e| e.error)
            .collect()
    }
}
