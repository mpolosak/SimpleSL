use std::rc::Rc;

use super::{
    local_variable::LocalVariables, recreate_instructions, traits::ExecResult, CreateInstruction,
    Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub elements: Rc<[Instruction]>,
}

impl CreateInstruction for Tuple {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let elements = pair
            .into_inner()
            .map(|pair| Instruction::new_expression(pair, interpreter, local_variables))
            .collect::<Result<Rc<[Instruction]>>>()?;
        Ok(Self::create_from_elements(elements))
    }
}

impl Tuple {
    fn create_from_elements(elements: Rc<[Instruction]>) -> Instruction {
        let mut array = Vec::new();
        for instruction in &*elements {
            let Instruction::Variable(variable) = instruction else {
                return Self { elements }.into();
            };
            array.push(variable.clone());
        }
        Instruction::Variable(Variable::Tuple(array.into()))
    }
}

impl Exec for Tuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.elements)?;
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for Tuple {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let elements = recreate_instructions(&self.elements, local_variables, interpreter)?;
        Ok(Self::create_from_elements(elements))
    }
}

impl ReturnType for Tuple {
    fn return_type(&self) -> Type {
        let types = self.elements.iter().map(Instruction::return_type).collect();
        Type::Tuple(types)
    }
}

impl From<Tuple> for Instruction {
    fn from(value: Tuple) -> Self {
        Instruction::Tuple(value)
    }
}
