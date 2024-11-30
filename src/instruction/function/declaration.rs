use crate::{
    instruction::{local_variable::LocalVariables, Instruction, InstructionWithStr},
    Error,
};
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
    let str = pair.as_str().into();
    let function = super::Function::create_instruction(pair, local_variables, Some(ident.clone()))?;
    local_variables.insert(ident.clone(), (&function).into());
    instructions.push(InstructionWithStr {
        instruction: function,
        str,
    });

    let str = format!("set {}", ident).into();
    let instruction = Instruction::Set(ident);
    instructions.push(InstructionWithStr { instruction, str });
    Ok(())
}
