use super::{
    local_variable::LocalVariables,
    traits::{Exec, ExecResult, Recreate},
    Instruction, InstructionWithStr,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Set {
    ident: Arc<str>,
    instruction: InstructionWithStr,
}

impl Set {
    pub fn new(
        ident: Arc<str>,
        instruction: InstructionWithStr,
        local_variables: &mut LocalVariables,
    ) -> Self {
        local_variables.insert(ident.clone(), (&instruction.instruction).into());
        Self { ident, instruction }
    }

    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let instruction = InstructionWithStr::new(pair, local_variables)?;
        Ok(Self::new(ident, instruction, local_variables).into())
    }
}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        interpreter.insert(self.ident.clone(), result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        local_variables.insert(self.ident.clone(), (&instruction.instruction).into());
        Ok(Self {
            ident: self.ident.clone(),
            instruction,
        }
        .into())
    }
}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
