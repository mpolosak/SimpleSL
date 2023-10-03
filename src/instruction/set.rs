use super::{
    local_variable::LocalVariables,
    traits::{BaseInstruction, Exec, Recreate},
    Instruction, MutCreateInstruction,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Result,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
pub struct Set {
    pub ident: Rc<str>,
    pub instruction: Instruction,
}

impl MutCreateInstruction for Set {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let ident: Rc<str> = inner.next().unwrap().as_str().into();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, interpreter, local_variables)?;
        local_variables.insert(ident.clone(), (&instruction).into());
        Ok(Self { ident, instruction }.into())
    }
}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
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
    ) -> Result<Instruction> {
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
