use super::{
    local_variable::LocalVariableMap,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable_type::{GetType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Set {
    ident: String,
    instruction: Box<Instruction>,
}

impl Set {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap().as_str().to_owned();
        let instruction = Instruction::new(variables, inner.next().unwrap(), local_variables)?;
        local_variables.insert(ident.clone(), instruction.clone().into());
        Ok(Self {
            ident,
            instruction: Box::new(instruction),
        })
    }
}

impl Exec for Set {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let result = self.instruction.exec(interpreter, local_variables)?;
        local_variables.insert(&self.ident, result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instruction = self.instruction.recreate(local_variables, args);
        local_variables.insert(self.ident.clone(), instruction.clone().into());
        Self {
            instruction: instruction.into(),
            ..self
        }
        .into()
    }
}

impl From<Set> for Instruction {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}

impl GetType for Set {
    fn get_type(&self) -> Type {
        self.instruction.get_type()
    }
}
