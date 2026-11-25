use super::{ExecStop, Instruction, local_variable::LocalVariables};
use crate::{
    Error,
    errors::WrongReturn,
    instruction::{BaseInstruction, Exec, Recreate, panic::Panic, parsing_context::ParsingContext},
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Return(pub Instruction);

impl Return {
    pub fn create_instruction(
        pair: Pair<Rule>,
        parsing_context: &mut ParsingContext,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let Some(function) = parsing_context.local_variables.function().cloned() else {
            errors.push(Error::ReturnOutsideFunction);
            return Panic.into();
        };
        let instruction = if let Some(pair) = pair.into_inner().next() {
            Instruction::new(pair, parsing_context, errors)
        } else {
            Variable::none().into()
        };
        let returned = instruction.return_type();
        if !returned.matches(function.return_type()) {
            errors.push(
                WrongReturn {
                    function_name: function.name(),
                    function_return_type: function.return_type().clone(),
                    returned,
                }
                .into(),
            );
        }
        Self(instruction).into()
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
