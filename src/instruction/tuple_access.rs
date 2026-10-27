use super::{Instruction, InstructionWithStr};
use crate::{
    instruction::{function::call::FunctionCall, tuple::Tuple}, stdlib::operators::At, variable::{from_str::parse_int, ReturnType, Variable}, Error
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(tuple: InstructionWithStr, op: Pair<Rule>) -> Result<Instruction, Error> {
    let tuple_type = tuple.return_type();
    let Some(len) = tuple_type.min_tuple_len() else {
        return Err(Error::CannotTupleAccess(tuple.str, tuple_type));
    };

    let pair = op.into_inner().next().unwrap();
    let index = parse_int(pair)? as usize;

    let Some(return_type) = tuple_type.tuple_element_at(index) else {
        return Err(Error::TupleIndexTooBig(index, tuple.str, len));
    };

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [tuple.instruction, Variable::from(index).into()].into(),
    }
    .into();

    Ok(FunctionCall { function, args }.into())
}
