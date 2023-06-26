use super::{
    exec_instructions,
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{error::Error, interpreter::VariableMap, parse::Rule, variable::Variable};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Array {
    instructions: Vec<Instruction>,
}

impl Array {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| Instruction::new(variables, arg, local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { instructions })
    }
}

impl Exec for Array {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let array = exec_instructions(&self.instructions, interpreter, local_variables)?;
        Ok(Variable::Array(array.into()))
    }
}

impl Recreate for Array {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instructions = recreate_instructions(self.instructions, local_variables, args);
        Self { instructions }.into()
    }
}

impl From<Array> for Instruction {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}
