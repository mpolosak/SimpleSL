use super::{Instruction, InstructionWithSpan};
use crate::{
    Error,
    errors::{CannotTupleAccess, TupleIndexTooBig},
    instruction::{
        function::call::FunctionCall, panic::Panic, parsing_context::ParsingContext, tuple::Tuple,
    },
    stdlib::operators::At,
    variable::{ReturnType, Type, Variable, from_str::parse_int},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(
    tuple: InstructionWithSpan,
    op: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<Error>,
) -> Instruction {
    let var_type = tuple.return_type();
    let Some(len) = var_type.min_tuple_len() else {
        let ins = parsing_context.slice(tuple.span).into();
        errors.push(CannotTupleAccess { ins, var_type }.into());
        return Panic.into();
    };

    let pair = op.into_inner().next().unwrap();
    let index = match parse_int(pair) {
        Ok(index) => index as usize,
        Err(error) => {
            errors.push(error);
            0
        }
    };

    let return_type = var_type.tuple_element_at(index).unwrap_or_else(|| {
        let ins = parsing_context.slice(tuple.span).into();
        errors.push(TupleIndexTooBig { index, ins, len }.into());
        Type::Never
    });

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [tuple.instruction, Variable::from(index).into()].into(),
    }
    .into();

    FunctionCall { function, args }.into()
}
