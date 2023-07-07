use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Block {
    instructions: Vec<Instruction>,
}

impl Block {
    pub fn new(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
        variables: &VariableMap,
    ) -> Result<Self, Error> {
        let mut local_variables = local_variables.clone();
        let instructions = pair
            .into_inner()
            .map(|pair| Instruction::new(variables, pair, &mut local_variables))
            .collect::<Result<Vec<Instruction>, Error>>()?;
        Ok(Self { instructions })
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
