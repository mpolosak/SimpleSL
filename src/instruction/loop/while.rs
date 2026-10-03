use crate::{
    Error,
    instruction::{
        Instruction, Loop, control_flow::IfElse, local_variable::LocalVariables,
    },
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let mut inner = pair.into_inner();
    let condition_pair = inner.next().unwrap();
    let condition_str = condition_pair.as_str().into();
    let condition = Instruction::new(condition_pair, local_variables)?;
    let return_type = condition.return_type();
    if return_type != Type::Bool {
        return Err(Error::WrongCondition(condition_str, return_type));
    }
    let in_loop = local_variables.in_loop;
    local_variables.in_loop = true;
    let instruction = Instruction::new(inner.next().unwrap(), local_variables)?;
    local_variables.in_loop = in_loop;
    if let Instruction::Variable(value) = condition {
        return if value == Variable::Bool(true) {
            Ok(Loop(instruction).into())
        } else {
            Ok(Variable::Void.into())
        };
    }
    let instruction = IfElse {
        condition,
        if_true: instruction,
        if_false: Instruction::Break,
    }
    .into();

    Ok(Loop(instruction).into())
}
