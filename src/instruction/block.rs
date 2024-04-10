use super::{
    local_variable::LocalVariables, recreate_instructions, traits::ExecResult, CreateInstruction,
    Exec, Instruction, InstructionWithStr, Recreate,
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
pub struct Block {
    instructions: Arc<[InstructionWithStr]>,
}

impl CreateInstruction for Block {
    fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.create_layer();
        let instructions = local_variables.create_instructions(pair.into_inner())?;
        if instructions.is_empty() {
            return Ok(Instruction::Variable(Variable::Void));
        }
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
