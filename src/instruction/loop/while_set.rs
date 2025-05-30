use crate::{
    Error,
    instruction::{
        Instruction, InstructionWithStr, Loop, control_flow::SetIfElse,
        local_variable::LocalVariables,
    },
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let str = pair.as_str();
    let in_loop = local_variables.in_loop;
    local_variables.in_loop = true;
    let mut set_if_else = SetIfElse::create(pair, local_variables)?;
    local_variables.in_loop = in_loop;
    set_if_else.else_instruction = InstructionWithStr {
        instruction: Instruction::Break,
        str: "break".into(),
    };
    let str = format!("if {} else break", str.strip_prefix("while").unwrap()).into();
    let instruction = InstructionWithStr {
        instruction: set_if_else.into(),
        str,
    };
    Ok(Loop(instruction).into())
}
