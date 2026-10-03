use crate::{
    Error,
    instruction::{
        Instruction, Loop, control_flow::SetIfElse,
        local_variable::LocalVariables,
    },
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let in_loop = local_variables.in_loop;
    local_variables.in_loop = true;
    let mut set_if_else = SetIfElse::create(pair, local_variables)?;
    local_variables.in_loop = in_loop;
    set_if_else.else_instruction = Instruction::Break;
    Ok(Loop(set_if_else.into()).into())
}
