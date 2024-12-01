use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    variable::{self, ReturnType, Type, Typed},
    Error, ExecError, Interpreter,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Mut {
    var_type: Type,
    value: Arc<[InstructionWithStr]>,
}

impl Mut {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        if pair.as_rule() == Rule::expr {
            let mut value = Vec::new();
            InstructionWithStr::create(pair, local_variables, &mut value)?;
            let value: Arc<[InstructionWithStr]> = value.into();
            let var_type = local_variables.result.as_ref().unwrap().as_type();
            return Ok(Mut { var_type, value }.into());
        }
        let var_type = Type::from(pair);

        let pair = inner.next().unwrap();
        let given = pair.as_str().into();
        let mut value = Vec::new();
        InstructionWithStr::create(pair, local_variables, &mut value)?;
        let value: Arc<[InstructionWithStr]> = value.into();
        let instruction_return_type = local_variables.result.as_ref().unwrap().as_type();
        if !instruction_return_type.matches(&var_type) {
            return Err(Error::WrongInitialization {
                declared: var_type,
                given,
                given_type: instruction_return_type,
            });
        }
        Ok(Mut { var_type, value }.into())
    }
}

impl Exec for Mut {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        interpreter.exec_all(&self.value)?;
        let variable = interpreter.result().unwrap().clone().into();
        Ok(variable::Mut {
            var_type: self.var_type.clone(),
            variable,
        }
        .into())
    }
}

impl Recreate for Mut {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let value = recreate_instructions(&self.value, local_variables)?;
        Ok(Mut {
            var_type: self.var_type.clone(),
            value,
        }
        .into())
    }
}

impl ReturnType for Mut {
    fn return_type(&self) -> Type {
        Type::Mut(self.var_type.clone().into())
    }
}
