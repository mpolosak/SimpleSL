use super::{local_variable::LocalVariables, Instruction, InstructionWithStr};
use crate::{
    variable::{ReturnType, Variable},
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
    instructions: &mut Vec<InstructionWithStr>,
) -> Result<(), Error> {
    let Some(function) = local_variables.function().cloned() else {
        return Err(Error::ReturnOutsideFunction);
    };
    if let Some(pair) = pair.into_inner().next() {
        InstructionWithStr::create(pair, local_variables, instructions)?
    } else {
        instructions.push(Variable::Void.into());
    };
    let returned = if let InstructionWithStr {
        instruction: Instruction::ExitScope,
        ..
    } = instructions.last().unwrap()
    {
        instructions[instructions.len() - 2].return_type()
    } else {
        instructions.last().unwrap().return_type()
    };
    if !returned.matches(function.return_type()) {
        return Err(Error::WrongReturn {
            function_name: function.name(),
            function_return_type: function.return_type().clone(),
            returned,
        });
    }
    instructions.push(InstructionWithStr {
        instruction: Instruction::Return,
        str: "return".into(),
    });
    Ok(())
}
