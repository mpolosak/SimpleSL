use super::{
    local_variable::LocalVariableMap, recreate_instructions, CreateInstruction, Exec, Instruction,
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
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.clone();
        let instructions = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, interpreter, &mut local_variables))
            .collect::<Result<Box<[Instruction]>, Error>>()?;
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        interpreter.add_layer();
        let mut result = Variable::Void;
        for instruction in self.instructions.iter() {
            result = instruction.exec(interpreter)?;
        }
        interpreter.remove_layer();
        Ok(result)
    }
}

impl Recreate for Block {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.clone();
        let instructions =
            recreate_instructions(&self.instructions, &mut local_variables, interpreter)?;
        Ok(Self { instructions }.into())
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
