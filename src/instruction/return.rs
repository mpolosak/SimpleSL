use super::{
    local_variable::LocalVariables,
    unary_operation::{UnaryOperation, UnaryOperator},
    Instruction,
};
use crate::{
    variable::{ReturnType, Variable},
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let Some(function) = local_variables.function().cloned() else {
        return Err(Error::ReturnOutsideFunction);
    };
    let instruction = if let Some(pair) = pair.into_inner().next() {
        Instruction::new(pair, local_variables)?
    } else {
        Variable::Void.into()
    };
    let returned = instruction.return_type();
    if !returned.matches(function.return_type()) {
        return Err(Error::WrongReturn {
            function_name: function.name(),
            function_return_type: function.return_type().clone(),
            returned,
        });
    }
    Ok(UnaryOperation {
        instruction,
        op: UnaryOperator::Return,
    }
    .into())
}
