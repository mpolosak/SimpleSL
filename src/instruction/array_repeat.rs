use super::{Instruction, InstructionWithSpan};
use crate::errors::{ErrorWithSpan, WrongLengthType};
use crate::instruction::parsing_context::ParsingContext;
use crate::instruction::{function::call::FunctionCall, tuple::Tuple};
use crate::{
    self as simplesl,
    stdlib::operators::ArrayRepeat,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let mut inner = pair.into_inner();
    let value = InstructionWithSpan::new_expression(inner.next().unwrap(), parsing_context, errors)
        .instruction;
    let len = InstructionWithSpan::new_expression(inner.next().unwrap(), parsing_context, errors);
    if !len.return_type().matches(&Type::Int) {
        let span = len.span;
        let ins = parsing_context.slice(span).into();
        let error = WrongLengthType { ins }.into();
        errors.push(ErrorWithSpan { error, span });
    }

    let value_type = value.return_type();
    let mut function = Arc::unwrap_or_clone(Variable::from(ArrayRepeat).into_function().unwrap());
    function.return_type = var_type!([value_type]);

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [value, len.instruction].into(),
    }
    .into();

    FunctionCall { function, args }.into()
}
