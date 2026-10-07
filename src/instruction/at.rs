use std::sync::Arc;

use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    instruction::{block::Block, set::Set, unary_operation::function_call}, stdlib::operators::At, variable::{ReturnType, Type, Variable}, Error
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    lhs: Instruction,
    index: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let pair = index.into_inner().next().unwrap();
    let index = InstructionWithStr::new_expression(pair, local_variables)?;
    let instruction_return_type = lhs.return_type();
    if index.return_type() != Type::Int {
        return Err(Error::CannotIndexWith(index.str));
    }
    if !instruction_return_type.can_be_indexed() {
        return Err(Error::CannotIndexInto(instruction_return_type));
    }

    let return_type = lhs.return_type().index_result().unwrap();
    let variable_set = Set::new_ident("variable".into(), lhs).into();
    let index_set = Set::new_ident("index".into(), index.instruction).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;
    let call = function_call(function);

    Ok(Block{ instructions:  [variable_set, index_set, call].into()}.into())
}
