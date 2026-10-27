use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::instruction::{function::call::FunctionCall, tuple::Tuple};
use crate::{
    self as simplesl, Error,
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
    let mut function = Arc::unwrap_or_clone(Variable::from(ArrayRepeat).into_function().unwrap());
    function.return_type = var_type!([value_type]);

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [value, len.instruction].into(),
    }
    .into();

    Ok(FunctionCall { function, args }.into())
}
