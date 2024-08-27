use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Block {
    instructions: Arc<[InstructionWithStr]>,
}

impl Block {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.create_layer();
        let instructions = local_variables.create_instructions(pair.into_inner())?;
        Ok(Self { instructions }.into())
    }
}

impl Exec for Block {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut interpreter = interpreter.create_layer();
        Ok(interpreter
            .exec(&self.instructions)?
            .last()
            .cloned()
            .unwrap_or(Variable::Void))
    }
}

impl Recreate for Block {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let mut local_variables = local_variables.create_layer();
        let instructions = recreate_instructions(&self.instructions, &mut local_variables)?;
        Ok(Self { instructions }.into())
    }
}

impl ReturnType for Block {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}
