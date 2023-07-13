use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Block {
    instructions: Vec<Instruction>,
}

impl CreateInstruction for Block {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut local_variables = local_variables.clone();
        let instructions = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, variables, &mut local_variables))
            .collect::<Result<Vec<Instruction>, Error>>()?;
        if instructions
            .iter()
            .all(|instruction| matches!(instruction, Instruction::Variable(_)))
        {
            if let Some(Instruction::Variable(variable)) = instructions.last() {
                Ok(Instruction::Variable(variable.clone()))
            } else {
                Ok(Instruction::Variable(Variable::Null))
            }
        } else {
            Ok(Self { instructions }.into())
        }
    }
}

impl Exec for Block {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let mut local_variables = local_variables.clone();
        let mut result = Variable::Null;
        for instruction in &self.instructions {
            result = instruction.exec(interpreter, &mut local_variables)?;
        }
        Ok(result)
    }
}

impl Recreate for Block {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let mut local_variables = local_variables.clone();
        let instructions = self
            .instructions
            .into_iter()
            .map(|instruction| instruction.recreate(&mut local_variables, args))
            .collect();
        Self { instructions }.into()
    }
}

impl GetReturnType for Block {
    fn get_return_type(&self) -> Type {
        match self.instructions.last() {
            Some(last) => last.get_return_type(),
            None => Type::Null,
        }
    }
}

impl From<Block> for Instruction {
    fn from(value: Block) -> Self {
        Self::Block(value)
    }
}
