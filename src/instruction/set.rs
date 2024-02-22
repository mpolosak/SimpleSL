use super::{
    local_variable::LocalVariables,
    traits::{BaseInstruction, Exec, ExecResult, Recreate},
    Instruction, MutCreateInstruction,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
pub struct Set {
    ident: Rc<str>,
    instruction: Instruction,
}

impl Set {
    pub fn new(
        ident: Rc<str>,
        instruction: Instruction,
        local_variables: &mut LocalVariables,
    ) -> Self {
        local_variables.insert(ident.clone(), (&instruction).into());
        Self { ident, instruction }
    }
}

impl MutCreateInstruction for Set {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
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
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        local_variables.insert(self.ident.clone(), (&instruction).into());
        Ok(Self {
            ident: self.ident.clone(),
            instruction,
        }
        .into())
    }
}

impl BaseInstruction for Set {}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
