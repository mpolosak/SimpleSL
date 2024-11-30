use std::sync::Arc;

use super::{Instruction, InstructionWithStr, Loop};
use crate::{
    instruction::{
        control_flow::SetIfElse,
        local_variable::{LocalVariable, LocalVariables},
    },
    variable::Type,
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let str = pair.as_str().into();
    let mut loop_instructions = Vec::<InstructionWithStr>::new();

    let mut inner = pair.into_inner();
    let ident: Arc<str> = inner.next().unwrap().as_str().into();
    let pair = inner.next().unwrap();
    let var_type = Type::from(pair);
    let pair = inner.next().unwrap();
    InstructionWithStr::create(pair, local_variables, instructions)?;

    local_variables.in_loop = true;
    let if_match_pair = inner.next().unwrap();
    let mut if_match = Vec::<InstructionWithStr>::new();
    local_variables.new_layer();
    local_variables.insert(ident.clone(), LocalVariable::Other(var_type.clone()));
    InstructionWithStr::create(if_match_pair, local_variables, &mut if_match)?;
    local_variables.drop_layer();
    let if_match = if_match.into();
    local_variables.in_loop = false;

    let else_instructions = [InstructionWithStr {
        instruction: Instruction::Break,
        str: "break".into(),
    }]
    .into();

    let instruction = SetIfElse {
        ident,
        var_type,
        if_match,
        else_instructions,
    }
    .into();
    let instruction = InstructionWithStr {
        instruction,
        str: "if else".into(),
    };
    loop_instructions.push(instruction);

    let instruction = Loop(loop_instructions.into()).into();
    instructions.push(InstructionWithStr { instruction, str });
    Ok(())
}
