use super::{
    local_variable::LocalVariables,
    traits::{Exec, ExecResult, Recreate},
    Instruction, InstructionWithStr, MutCreateInstruction,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
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
}

impl MutCreateInstruction for Set {
    fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<InstructionWithStr, Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let ident: Arc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let instruction = InstructionWithStr::new(pair, local_variables)?;
        let instruction = Self::new(ident, instruction, local_variables).into();
        Ok(InstructionWithStr { instruction, str })
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
