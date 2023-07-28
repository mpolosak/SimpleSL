use super::{
    exec_instructions, local_variable::LocalVariables, recreate_instructions, CreateInstruction,
    Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Tuple {
    pub elements: Box<[Instruction]>,
}

impl CreateInstruction for Tuple {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| Instruction::new(pair, interpreter, local_variables))
            .collect::<Result<Box<[Instruction]>, Error>>()?;
        Ok(Self::create_from_elements(elements))
    }
}

impl Tuple {
    fn create_from_elements(elements: Box<[Instruction]>) -> Instruction {
        let mut array = Vec::new();
        for instruction in elements.iter() {
            let Instruction::Variable(variable) = instruction else {
                return Self { elements }.into();
            };
            array.push(variable.clone());
        }
        Instruction::Variable(Variable::Tuple(array.into()))
    }
}

impl Exec for Tuple {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let elements = exec_instructions(&self.elements, interpreter)?;
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for Tuple {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let elements = recreate_instructions(&self.elements, local_variables, interpreter)?;
        Ok(Self::create_from_elements(elements))
    }
}

impl GetReturnType for Tuple {
    fn get_return_type(&self) -> Type {
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
