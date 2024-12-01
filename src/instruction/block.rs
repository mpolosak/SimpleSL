use super::{
    local_variable::{LocalVariable, LocalVariables},
    Instruction, InstructionWithStr,
};
use crate::{variable::Variable, Error};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let pairs = pair.into_inner();
    if pairs.len() == 0 {
        instructions.push(Variable::Void.into());
        local_variables.result = Some(LocalVariable::Variable(Variable::Void));
        return Ok(());
    }
    local_variables.new_layer();
    instructions.push(InstructionWithStr {
        instruction: Instruction::EnterScope,
        str: "{".into(),
    });
    for pair in pairs {
        InstructionWithStr::create(pair, local_variables, instructions)?;
    }
    instructions.push(InstructionWithStr {
        instruction: Instruction::ExitScope,
        str: "}".into(),
    });
    local_variables.drop_layer();
    Ok(())
}
