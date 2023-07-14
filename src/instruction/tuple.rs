use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Tuple {
    pub elements: Vec<Instruction>,
}

impl CreateInstruction for Tuple {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, variables, local_variables))
            .collect::<Result<Vec<Instruction>, Error>>()?;
        Ok(Self::create_from_elements(elements))
    }
}

impl Tuple {
    fn create_from_elements(elements: Vec<Instruction>) -> Instruction {
        let mut array = Vec::new();
        for instruction in &elements {
            if let Instruction::Variable(variable) = instruction {
                array.push(variable.clone());
            } else {
                return Self { elements }.into();
            }
        }
        Instruction::Variable(Variable::Tuple(array.into()))
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
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let elements = self
            .elements
            .into_iter()
            .map(|instruction| instruction.recreate(local_variables, args))
            .collect::<Result<Vec<Instruction>, Error>>()?;
        Ok(Self::create_from_elements(elements))
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
