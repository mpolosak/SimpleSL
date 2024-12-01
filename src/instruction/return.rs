use super::{
    local_variable::{LocalVariable, LocalVariables},
    Instruction, InstructionWithStr,
};
use crate::{
    variable::{Typed, Variable},
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
        InstructionWithStr::create(pair, local_variables, instructions)?;
    } else {
        instructions.push(Variable::Void.into());
        local_variables.result = Some(LocalVariable::Variable(Variable::Void));
    };
    let returned = local_variables.result.as_ref().unwrap().as_type();
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
