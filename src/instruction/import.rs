use super::{
    local_variable::LocalVariables,
    recreate_instructions,
    traits::{ExecResult, MutCreateInstruction},
    Exec, Instruction, Recreate,
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
    instructions: Arc<[Instruction]>,
}

impl MutCreateInstruction for Import {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let Variable::String(path) = Variable::try_from(pair.into_inner().next().unwrap())? else {
            unreachable!()
        };
        let instructions = interpreter.load(&path, local_variables)?;
        if instructions.is_empty() {
            return Ok(Instruction::Variable(Variable::Void));
        }
        if let [element] = instructions.as_ref() {
            return Ok(element.clone());
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
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.instructions, local_variables, interpreter)?;
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
