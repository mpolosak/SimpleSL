use super::{
    local_variable::LocalVariables, Exec, ExecResult, ExecStop, Instruction, InstructionWithStr,
    Recreate,
};
use crate::{
    variable::{ReturnType, Type, Variable},
    Error, ExecError, Interpreter,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct While {
    condition: InstructionWithStr,
    instruction: InstructionWithStr,
}

impl While {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let condition = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
        let return_type = condition.return_type();
        if return_type != Type::Bool {
            return Err(Error::WrongCondition(condition.str, return_type));
        }
        local_variables.in_loop = true;
        let instruction = InstructionWithStr::new(inner.next().unwrap(), local_variables)?;
        local_variables.in_loop = false;
        Ok(Self {
            condition,
            instruction,
        }
        .into())
    }
}

impl Exec for While {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        while self.condition.exec(interpreter)?.into_bool().unwrap() {
            match self.instruction.exec(interpreter) {
                Ok(_) | Err(ExecStop::Continue) => (),
                Err(ExecStop::Break) => break,
                e => return e,
            }
        }
        Ok(Variable::Void)
    }
}

impl Recreate for While {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let condition = self.condition.recreate(local_variables)?;
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(Self {
            condition,
            instruction,
        }
        .into())
    }
}

impl From<While> for Instruction {
    fn from(value: While) -> Self {
        Self::While(value.into())
    }
}
