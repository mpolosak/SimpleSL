use super::{
    local_variable::LocalVariables, Instruction, InstructionWithStr,
};
use crate::Error;
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let ident: Arc<str> = inner.next().unwrap().as_str().into();
    let pair = inner.next().unwrap();
    InstructionWithStr::create(pair, local_variables, instructions)?;
    let value = &if let InstructionWithStr{instruction: Instruction::ExitScope, ..} = instructions.last().unwrap() {
        &instructions[instructions.len()-2]
    } else {
        instructions.last().unwrap()
    }.instruction;
    local_variables.insert(ident.clone(), value.into());
    let str = ("set ".to_owned() + &ident).into();
    instructions.push(InstructionWithStr{ instruction: Instruction::Set(ident), str });
    Ok(())
}
