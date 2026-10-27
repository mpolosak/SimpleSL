use crate::{
    Error,
    instruction::{
        Instruction, Loop, control_flow::If, local_variable::LocalVariables, r#loop::Break,
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
    let mut if_ins = If::create(pair, local_variables)?;
    local_variables.in_loop = in_loop;
    if_ins.else_instruction = Break.into();
    Ok(Loop(if_ins.into()).into())
}
