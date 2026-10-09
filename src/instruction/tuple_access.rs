use super::{Instruction, InstructionWithStr};
use crate::{
    Error,
    instruction::{block::Block, postfix_op::function_call, set::Set},
    stdlib::operators::At,
    variable::{ReturnType, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(tuple: InstructionWithStr, op: Pair<Rule>) -> Result<Instruction, Error> {
    let return_type = tuple.return_type();
    if !return_type.is_tuple() {
        return Err(Error::CannotTupleAccess(tuple.str, return_type));
    }
    let pair = op.into_inner().next().unwrap();
    let index_var = Variable::try_from(pair)?;
    let index = *index_var.as_int().unwrap() as usize;
    let len = return_type.min_tuple_len().unwrap();
    if index >= len {
        return Err(Error::TupleIndexTooBig(index, tuple.str, len));
    }

    let return_type = tuple.return_type().tuple_element_at(index).unwrap();

    let tuple_set = Set::new_ident("variable".into(), tuple.instruction).into();
    let len_set = Set::new_ident("index".into(), index_var.into()).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;
    let call = function_call(function);

    Ok(Block {
        instructions: [tuple_set, len_set, call].into(),
    }
    .into())
}
