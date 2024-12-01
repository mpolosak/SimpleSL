use super::{
    control_flow::if_else::return_type, local_variable::LocalVariables, recreate_instructions,
    Exec, ExecResult, ExecStop, Instruction, InstructionWithStr, Recreate,
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
    pub elements: Arc<[Arc<[InstructionWithStr]>]>,
}

impl Tuple {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| {
                let mut value = Vec::new();
                InstructionWithStr::create(pair, local_variables, &mut value)?;
                Ok(value.into())
            })
            .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, Error>>()?;
        Ok(Self { elements }.into())
    }
}

impl Exec for Tuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = self
            .elements
            .iter()
            .map(|instructions| {
                interpreter.exec_all(instructions)?;
                Ok(interpreter.result().unwrap().clone())
            })
            .collect::<Result<Arc<[Variable]>, ExecStop>>()?;
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for Tuple {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let elements = self
            .elements
            .iter()
            .map(|instructions| recreate_instructions(instructions, local_variables))
            .collect::<Result<Arc<[Arc<[InstructionWithStr]>]>, ExecError>>()?;
        Ok(Self { elements }.into())
    }
}

impl ReturnType for Tuple {
    fn return_type(&self) -> Type {
        let types = self
            .elements
            .iter()
            .map(Arc::as_ref)
            .map(return_type)
            .collect();
        Type::Tuple(types)
    }
}
