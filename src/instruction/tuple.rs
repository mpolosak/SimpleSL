use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub elements: Arc<[InstructionWithStr]>,
}

impl Tuple {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| InstructionWithStr::new_expression(pair, local_variables))
            .collect::<Result<Arc<[InstructionWithStr]>, Error>>()?;
        Ok(Self { elements }.into())
    }

    fn create_from_elements(elements: Arc<[InstructionWithStr]>) -> Instruction {
        let mut array = Vec::new();
        for instruction in &*elements {
            let InstructionWithStr {
                instruction: Instruction::Variable(variable),
                ..
            } = instruction
            else {
                return Self { elements }.into();
            };
            array.push(variable.clone());
        }
        Instruction::Variable(Variable::Tuple(array.into()))
    }
}

impl Exec for Tuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec_all(&self.elements)?;
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for Tuple {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let elements = recreate_instructions(&self.elements, local_variables)?;
        Ok(Self::create_from_elements(elements))
    }
}

impl ReturnType for Tuple {
    fn return_type(&self) -> Type {
        let types = self.elements.iter().map(ReturnType::return_type).collect();
        Type::Tuple(types)
    }
}
