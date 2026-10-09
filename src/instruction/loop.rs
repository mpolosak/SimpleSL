pub mod r#for;
pub mod r#while;
use super::{Exec, ExecResult, ExecStop, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    Error, Interpreter,
    instruction::BaseInstruction,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Loop(pub Instruction);

impl Loop {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let in_loop = local_variables.in_loop;
        local_variables.in_loop = true;
        let instruction = Instruction::new(inner.next().unwrap(), local_variables)?;
        local_variables.in_loop = in_loop;
        Ok(Self(instruction).into())
    }
}

impl BaseInstruction for Loop {}

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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        Self(instruction).into()
    }
}

impl ReturnType for Loop {
    fn return_type(&self) -> Type {
        Type::Void
    }
}
