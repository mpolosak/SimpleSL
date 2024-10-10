use super::{
    local_variable::LocalVariables, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    variable::{self, ReturnType, Type},
    Error, ExecError, Interpreter,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Mut {
    var_type: Type,
    instruction: InstructionWithStr,
}

impl Mut {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        if pair.as_rule() == Rule::expr {
            let instruction = InstructionWithStr::new_expression(pair, local_variables)?;
            let var_type = instruction.return_type();
            return Ok(Mut {
                var_type,
                instruction,
            }
            .into());
        }
        let var_type = Type::from(pair);
        let pair = inner.next().unwrap();
        let instruction = InstructionWithStr::new_expression(pair, local_variables)?;
        let instruction_return_type = instruction.return_type();
        if !instruction_return_type.matches(&var_type) {
            return Err(Error::WrongInitialization {
                declared: var_type,
                given: instruction.str,
                given_type: instruction_return_type,
            });
        }
        Ok(Mut {
            var_type,
            instruction,
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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(Mut {
            var_type: self.var_type.clone(),
            instruction,
        }
        .into())
    }
}

impl ReturnType for Mut {
    fn return_type(&self) -> Type {
        Type::Mut(self.var_type.clone().into())
    }
}
