mod r#for;
pub mod r#while;
pub mod while_set;
use std::sync::Arc;

use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, ExecStop, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{variable::Variable, Error, ExecError, Interpreter};
use pest::iterators::Pair;
pub use r#for::For;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Loop(pub Arc<[InstructionWithStr]>);

impl Loop {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let str = pair.as_str().into();
        let pair = pair.into_inner().next().unwrap();
        let in_loop = local_variables.in_loop;
        local_variables.in_loop = true;
        let mut inner = Vec::<InstructionWithStr>::new();
        InstructionWithStr::create(pair, local_variables, &mut inner)?;
        local_variables.in_loop = in_loop;
        let instruction = Self(inner.into()).into();
        instructions.push(InstructionWithStr { instruction, str });
        Ok(())
    }
}

impl Exec for Loop {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        loop {
            match interpreter.exec_all(&self.0) {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(Variable::Void)
    }
}

impl Recreate for Loop {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.0, local_variables)?;
        Ok(Self(instructions).into())
    }
}

impl From<Loop> for Instruction {
    fn from(value: Loop) -> Self {
        Self::Loop(value.into())
    }
}
