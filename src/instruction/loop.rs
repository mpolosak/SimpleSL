use super::{
    local_variable::LocalVariables, Exec, ExecResult, ExecStop, Instruction, InstructionWithStr,
    Recreate,
};
use crate::{variable::Variable, Error, ExecError, Interpreter};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Loop(InstructionWithStr);

impl Loop {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        local_variables.in_loop = true;
        let instruction = InstructionWithStr::new(inner.next().unwrap(), local_variables)?;
        local_variables.in_loop = false;
        Ok(Self(instruction).into())
    }
}

impl Exec for Loop {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        loop {
            match self.0.exec(interpreter) {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                e => return e,
            }
        }
        Ok(Variable::Void)
    }
}

impl Recreate for Loop {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.0.recreate(local_variables)?;
        Ok(Self(instruction).into())
    }
}

impl From<Loop> for Instruction {
    fn from(value: Loop) -> Self {
        Self::Loop(value.into())
    }
}
