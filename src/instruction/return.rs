use super::{
    local_variable::LocalVariables,
    traits::{ExecResult, ExecStop},
    Exec, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError, Interpreter,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Return {
    instruction: InstructionWithStr,
}

impl Return {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let Some(function) = local_variables.function().cloned() else {
            return Err(Error::ReturnOutsideFunction);
        };
        let instruction = if let Some(pair) = pair.into_inner().next() {
            InstructionWithStr::new(pair, local_variables)?
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
        Ok(Self { instruction }.into())
    }
}

impl Exec for Return {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let value = self.instruction.exec(interpreter)?;
        Err(ExecStop::Return(value))
    }
}

impl Recreate for Return {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(Return { instruction }.into())
    }
}

impl ReturnType for Return {
    fn return_type(&self) -> Type {
        Type::Never
    }
}
