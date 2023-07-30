use super::{
    local_variable::LocalVariables, recreate_instructions, CreateInstruction, Exec, Instruction,
    Recreate,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Block {
    instructions: Box<[Instruction]>,
}

impl CreateInstruction for Block {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        local_variables.add_layer();
        let instructions = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, interpreter, local_variables))
            .collect::<Result<Box<[Instruction]>, Error>>()?;
        let result = if instructions
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
        };
        local_variables.remove_layer();
        result
    }
}

impl Exec for Block {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        interpreter.add_layer();
        let result = interpreter.exec(&self.instructions);
        interpreter.remove_layer();
        result
    }
}

impl Recreate for Block {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        local_variables.add_layer();
        let instructions = recreate_instructions(&self.instructions, local_variables, interpreter)?;
        let result = Ok(Self { instructions }.into());
        local_variables.remove_layer();
        result
    }
}

impl GetReturnType for Block {
    fn get_return_type(&self) -> Type {
        match self.instructions.last() {
            Some(last) => last.get_return_type(),
            None => Type::Void,
        }
    }
}

impl From<Block> for Instruction {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
