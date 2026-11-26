use super::{Instruction, InstructionWithRange};
use crate::{
    errors::{CannotTupleAccess, ErrorWithRange, TupleIndexTooBig},
    instruction::{
        function::call::FunctionCall, panic::Panic, parsing_context::ParsingContext, tuple::Tuple,
    },
    range_from_pest_span,
    stdlib::operators::At,
    variable::{ReturnType, Type, Variable, from_str::parse_int},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(
    tuple: InstructionWithRange,
    op: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithRange>,
) -> Instruction {
    let var_type = tuple.return_type();
    let Some(len) = var_type.min_tuple_len() else {
        let range = tuple.range;
        let ins = parsing_context.slice(range.clone()).into();
        let error = CannotTupleAccess { ins, var_type }.into();
        errors.push(ErrorWithRange { error, range });
        return Panic.into();
    };

    let pair = op.into_inner().next().unwrap();
    let span = pair.as_span();
    let index = match parse_int(pair) {
        Ok(index) => index as usize,
        Err(error) => {
            let span = range_from_pest_span(&span);
            errors.push(ErrorWithRange { error, range: span });
            0
        }
    };

    let return_type = var_type.tuple_element_at(index).unwrap_or_else(|| {
        let range = tuple.range;
        let ins = parsing_context.slice(range.clone()).into();
        let error = TupleIndexTooBig { index, ins, len }.into();
        errors.push(ErrorWithRange { error, range });
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
