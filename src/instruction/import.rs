use super::{
    local_variable::LocalVariables, recreate_instructions, traits::ExecResult, Exec, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::sync::Arc;

#[derive(Debug)]
pub struct Import {
    instructions: Arc<[InstructionWithStr]>,
}

impl Import {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let path = Variable::try_from(pair.into_inner().next().unwrap())?
            .into_string()
            .unwrap();
        let instructions = local_variables.load(&path)?;
        if instructions.is_empty() {
            return Ok(Variable::Void.into());
        }
        if let [element] = instructions.as_ref() {
            return Ok(element.instruction.clone());
        }
        Ok(Self { instructions }.into())
    }
}

impl Exec for Import {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        Ok(interpreter
            .exec(&self.instructions)?
            .last()
            .cloned()
            .unwrap_or(Variable::Void))
    }
}

impl Recreate for Import {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
        Ok(Self { instructions }.into())
    }
}

impl ReturnType for Import {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}
