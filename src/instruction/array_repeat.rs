use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    self as simplesl,
    instruction::{block::Block, unary_operation::function_call},
};
use crate::{
    Error,
    instruction::set::Set,
    stdlib::operators::ArrayRepeat,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let mut inner = pair.into_inner();
    let value =
        InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?.instruction;
    let len = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
    if !len.return_type().matches(&Type::Int) {
        return Err(Error::WrongLengthType(len.str));
    }

    let value_type = value.return_type();
    let value_set = Set::new_ident("value".into(), value).into();
    let len_set = Set::new_ident("len".into(), len.instruction).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(ArrayRepeat).into_function().unwrap());
    function.return_type = var_type!([value_type]);

    let call = function_call(function);

    Ok(Block {
        instructions: [value_set, len_set, call].into(),
    }
    .into())
}
