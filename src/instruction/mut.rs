use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, Interpreter,
    instruction::BaseInstruction,
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
        rhs: InstructionWithStr,
    ) -> Result<Instruction, Error> {
        let mut inner = op.into_inner();
        let Some(type_pair) = inner.next() else {
            return Ok(Mut {
                var_type: rhs.return_type(),
                instruction: rhs.instruction,
            }
            .into());
        };
        let var_type = Type::from(type_pair);
        let instruction_return_type = rhs.return_type();
        if !instruction_return_type.matches(&var_type) {
            return Err(Error::WrongInitialization {
                declared: var_type,
                given: rhs.str,
                given_type: instruction_return_type,
            });
        }
        Ok(Mut {
            var_type,
            instruction: rhs.instruction,
        }
        .into())
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
