use super::control_flow::if_else::return_type;
use super::ExecStop;
use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate as simplesl;
use crate::variable::Variable;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Arc<[Arc<[InstructionWithStr]>]>,
    pub element_type: Type,
}

impl Array {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let inner = pair.into_inner();
        let elements = inner
            .map(|pair| {
                let mut value = Vec::<InstructionWithStr>::new();
                InstructionWithStr::create(pair, local_variables, &mut value)?;
                Ok(value.into())
            })
            .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, Error>>()?;
        let element_type = elements
            .iter()
            .map(|instructions| return_type(instructions))
            .reduce(Type::concat)
            .unwrap_or(Type::Never);
        Ok(Self {
            elements,
            element_type,
        }
        .into())
    }
}

impl Exec for Array {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = self
            .elements
            .iter()
            .map(|instructions| {
                interpreter.exec_all(instructions)?;
                Ok(interpreter.result().unwrap().clone())
            })
            .collect::<Result<Arc<[Variable]>, ExecStop>>()?;
        Ok(elements.into())
    }
}

impl Recreate for Array {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let elements = self
            .elements
            .iter()
            .map(|instructions| recreate_instructions(instructions, local_variables))
            .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, ExecError>>()?;
        Ok(Self {
            elements,
            element_type: self.element_type.clone(),
        }
        .into())
    }
}

impl ReturnType for Array {
    fn return_type(&self) -> Type {
        let element_type = self.element_type.clone();
        var_type!([element_type])
    }
}
