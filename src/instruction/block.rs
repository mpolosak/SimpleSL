use super::{
    local_variable::LocalVariables, recreate_instructions, traits::BaseInstruction,
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
pub struct Block {
    instructions: Rc<[Instruction]>,
}

impl CreateInstruction for Block {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let mut local_variables = local_variables.create_layer();
        let instructions =
            interpreter.create_instructions(pair.into_inner(), &mut local_variables)?;
        if instructions.is_empty() {
            return Ok(Instruction::Variable(Variable::Void));
        }
        Ok(Self { instructions }.into())
    }
}

impl Exec for Block {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let mut interpreter = interpreter.create_layer();
        interpreter.exec(&self.instructions)
    }
}

impl Recreate for Block {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let mut local_variables = local_variables.create_layer();
        let instructions =
            recreate_instructions(&self.instructions, &mut local_variables, interpreter)?;
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

impl BaseInstruction for Block {}
