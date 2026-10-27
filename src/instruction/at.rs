use std::sync::Arc;

use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    Error,
    instruction::{function::call::FunctionCall, tuple::Tuple},
    stdlib::operators::At,
    variable::{ReturnType, Type, Variable},
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

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [lhs, index.instruction].into(),
    }
    .into();

    Ok(FunctionCall { function, args }.into())
}
