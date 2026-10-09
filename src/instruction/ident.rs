use crate::{
    Error,
    instruction::{
        BaseInstruction, Exec, Instruction, Recreate,
        local_variable::{LocalVariable, LocalVariables},
    },
    variable::{ReturnType, Typed},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Ident {
    pub ident: Arc<str>,
    pub variable: LocalVariable,
}

impl Ident {
    pub fn create_instruction(
        str: &Arc<str>,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Instruction, Error> {
        local_variables.get(str).map_or_else(
            || {
                local_variables
                    .interpreter
                    .get_variable(str)
                    .cloned()
                    .map(Instruction::from)
                    .ok_or_else(|| Error::VariableDoesntExist(str.clone()))
            },
            |var| {
                Ok(Self {
                    ident: str.clone(),
                    variable: var.clone(),
                }
                .into())
            },
        )
    }
}

impl BaseInstruction for Ident {}

impl Exec for Ident {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> super::ExecResult {
        interpreter
            .get_variable(&self.ident)
            .cloned()
            .ok_or_else(|| panic!("Tried to get variable {} that doest exist", self.ident))
    }
}

impl Recreate for Ident {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, crate::ExecError> {
        Ok(local_variables.get(&self.ident).map_or_else(
            || {
                local_variables
                    .interpreter
                    .get_variable(&self.ident)
                    .cloned()
                    .map(Instruction::from)
                    .unwrap_or_else(|| {
                        panic!("Tried to get variable {} that doest exist", self.ident)
                    })
            },
            |var| {
                Self {
                    ident: self.ident.clone(),
                    variable: var.clone(),
                }
                .into()
            },
        ))
    }
}

impl ReturnType for Ident {
    fn return_type(&self) -> crate::variable::Type {
        self.variable.as_type()
    }
}
