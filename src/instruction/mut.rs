use super::{
    Exec, ExecResult, Instruction, InstructionWithSpan, Recreate, local_variable::LocalVariables,
};
use crate::{
    Interpreter,
    errors::{ErrorWithSpan, WrongInitialization},
    instruction::{BaseInstruction, parsing_context::ParsingContext},
    variable::{self, ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Mut {
    var_type: Type,
    instruction: Instruction,
}

impl BaseInstruction for Mut {}

impl Mut {
    pub fn create_instruction(
        op: Pair<Rule>,
        rhs: InstructionWithSpan,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let mut inner = op.into_inner();
        let Some(type_pair) = inner.next() else {
            return Mut {
                var_type: rhs.return_type(),
                instruction: rhs.instruction,
            }
            .into();
        };
        let var_type = Type::from(type_pair);
        let instruction_return_type = rhs.return_type();
        if !instruction_return_type.matches(&var_type) {
            let span = rhs.span;
            let given = parsing_context.slice(span).into();
            let error = WrongInitialization {
                declared: var_type.clone(),
                given,
                given_type: instruction_return_type,
            }
            .into();
            errors.push(ErrorWithSpan { error, span });
        }
        Mut {
            var_type,
            instruction: rhs.instruction,
        }
        .into()
    }
}

impl Exec for Mut {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let variable = self.instruction.exec(interpreter)?.into();
        Ok(variable::Mut {
            var_type: self.var_type.clone(),
            variable,
        }
        .into())
    }
}

impl Recreate for Mut {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.instruction.recreate(local_variables);
        Mut {
            var_type: self.var_type.clone(),
            instruction,
        }
        .into()
    }
}

impl ReturnType for Mut {
    fn return_type(&self) -> Type {
        Type::Mut(self.var_type.clone().into())
    }
}
