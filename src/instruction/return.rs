use super::{
    local_variable::LocalVariables,
    traits::{BaseInstruction, ExecResult, ExecStop},
    Exec, Instruction, MutCreateInstruction, Recreate,
};
use crate::{
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, Interpreter, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Return {
    instruction: Instruction,
}

impl MutCreateInstruction for Return {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let Some(function) = local_variables.function().cloned() else {
            return Err(Error::ReturnOutsideFunction);
        };
        let instruction = if let Some(pair) = pair.into_inner().next() {
            Instruction::new(pair, interpreter, local_variables)?
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
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        Ok(Return { instruction }.into())
    }
}

impl ReturnType for Return {
    fn return_type(&self) -> Type {
        Type::Never
    }
}

impl BaseInstruction for Return {}
