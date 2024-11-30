use super::{Instruction, InstructionWithStr, Loop};
use crate::{
    instruction::{control_flow::SetIfElse, local_variable::LocalVariables},
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let str = pair.as_str();
    local_variables.in_loop = true;
    let mut set_if_else = SetIfElse::create(pair, local_variables)?;
    local_variables.in_loop = false;
    set_if_else.else_instruction = InstructionWithStr {
        instruction: Instruction::Break,
        str: "break".into(),
    };
    let if_else_str = format!("if {} else break", str.strip_prefix("while").unwrap()).into();
    let instruction = InstructionWithStr {
        instruction: set_if_else.into(),
        str: if_else_str,
    };
    let instruction = Loop([instruction].into()).into();
    instructions.push(InstructionWithStr{ instruction, str: str.into() });
    Ok(())
}
