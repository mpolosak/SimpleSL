use super::{ExecStop, Instruction, local_variable::LocalVariables};
use crate::{
    Error,
    instruction::{BaseInstruction, Exec, Recreate},
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Return(pub Instruction);

impl Return {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let Some(function) = local_variables.function().cloned() else {
            return Err(Error::ReturnOutsideFunction);
        };
        let instruction = if let Some(pair) = pair.into_inner().next() {
            Instruction::new(pair, local_variables)?
        } else {
            Variable::Void.into()
        };
        let returned = instruction.return_type();
        if !returned.matches(function.return_type()) {
            return Err(Error::WrongReturn {
                function_name: function.name(),
                function_return_type: function.return_type().clone(),
                returned,
            });
        }
        Ok(Self(instruction).into())
    }
}

impl BaseInstruction for Return {}

impl Exec for Return {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> super::ExecResult {
        let variable = self.0.exec(interpreter)?;
        Err(ExecStop::Return(variable))
    }
}

impl Recreate for Return {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        Self(instruction).into()
    }
}

impl ReturnType for Return {
    fn return_type(&self) -> Type {
        Type::Never
    }
}
