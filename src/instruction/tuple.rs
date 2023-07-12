use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Tuple {
    elements: Vec<Instruction>,
}

impl Tuple {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, variables, local_variables))
            .collect::<Result<Vec<Instruction>, Error>>()?;
        Ok(Self { elements })
    }
}

impl Exec for Tuple {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let mut elements = Vec::new();
        for element in &self.elements {
            elements.push(element.exec(interpreter, local_variables)?);
        }
        Ok(Variable::Tuple(elements.into()))
    }
}

impl Recreate for Tuple {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let elements = self
            .elements
            .into_iter()
            .map(|instruction| instruction.recreate(local_variables, args))
            .collect();
        Self { elements }.into()
    }
}

impl GetReturnType for Tuple {
    fn get_return_type(&self) -> crate::variable::Type {
        let types = self
            .elements
            .iter()
            .map(Instruction::get_return_type)
            .collect();
        Type::Tuple(types)
    }
}

impl From<Tuple> for Instruction {
    fn from(value: Tuple) -> Self {
        Instruction::Tuple(value)
    }
}
