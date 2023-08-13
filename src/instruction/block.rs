use std::rc::Rc;

use super::{
    local_variable::LocalVariables, recreate_instructions, CreateInstruction, Exec, Instruction,
    Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Block {
    instructions: Box<[Instruction]>,
}

impl CreateInstruction for Block {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut local_variables = local_variables.create_layer();
        let instructions = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, interpreter, &mut local_variables))
            .collect::<Result<Box<[Instruction]>>>()?;
        if instructions
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            if let Some(Instruction::Variable(variable)) = instructions.last() {
                Ok(Instruction::Variable(variable.clone()))
            } else {
                Ok(Instruction::Variable(Variable::Void))
            }
        } else {
            Ok(Self { instructions }.into())
        }
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

impl GetReturnType for Block {
    fn get_return_type(&self) -> Rc<Type> {
        match self.instructions.last() {
            Some(last) => last.get_return_type(),
            None => Type::Void.into(),
        }
    }
}

impl From<Block> for Instruction {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
