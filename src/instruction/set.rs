use super::{
    local_variable::LocalVariableMap,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::{GetReturnType, Type},
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Set {
    ident: Rc<str>,
    instruction: Instruction,
}

impl Set {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, variables, local_variables)?;
        local_variables.insert(ident.clone(), instruction.clone().into());
        Ok(Self { ident, instruction })
    }
}

impl Exec for Set {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let result = self.instruction.exec(interpreter, local_variables)?;
        local_variables.insert(self.ident.clone(), result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let instruction = self.instruction.recreate(local_variables, args)?;
        local_variables.insert(self.ident.clone(), instruction.clone().into());
        Ok(Self {
            ident: self.ident.clone(),
            instruction,
        }
        .into())
    }
}

impl From<Set> for Instruction {
    fn from(value: Set) -> Self {
        Self::Set(value.into())
    }
}

impl GetReturnType for Set {
    fn get_return_type(&self) -> Type {
        self.instruction.get_return_type()
    }
}
