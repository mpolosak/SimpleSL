use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
    recreate_instructions,
};
use crate::{
    self as simplesl, Error, ExecError,
    interpreter::Interpreter,
    variable::{ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Array {
    pub instructions: Arc<[InstructionWithStr]>,
    pub element_type: Type,
}

impl Array {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| InstructionWithStr::new_expression(arg, local_variables))
            .collect::<Result<Arc<_>, Error>>()?;
        let element_type = instructions
            .iter()
            .map(ReturnType::return_type)
            .reduce(Type::concat)
            .unwrap_or(Type::Never);
        Ok(Self {
            instructions,
            element_type,
        }
        .into())
    }
}

impl Exec for Array {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.instructions)?;
        Ok(elements.into())
    }
}

impl Recreate for Array {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
        let mut array = Vec::new();
        for instruction in &*instructions {
            let InstructionWithStr {
                instruction: Instruction::Variable(variable),
                ..
            } = instruction
            else {
                return Ok(Self {
                    instructions,
                    element_type: self.element_type.clone(),
                }
                .into());
            };
            array.push(variable.clone());
        }
        Ok(Instruction::Variable(array.into()))
    }
}

impl ReturnType for Array {
    fn return_type(&self) -> Type {
        let element_type = self.element_type.clone();
        var_type!([element_type])
    }
}
