use crate::{
    instruction::{
        local_variable::{LocalVariable, LocalVariables}, BaseInstruction, Exec, Instruction, Recreate
    }, variable::{ReturnType, Type, Typed}, Error
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
        errors: &mut Vec<Error>
    ) -> Instruction {
        local_variables.get(str).map_or_else(
            || {
                local_variables
                    .interpreter
                    .get_variable(str)
                    .cloned()
                    .map(Instruction::from)
                    .unwrap_or_else(|| {errors.push(Error::VariableDoesntExist(str.clone())); Self {
                    ident: str.clone(),
                    variable: LocalVariable::Other(Type::Never),
                }
                .into()})
            },
            |var| {
                Self {
                    ident: str.clone(),
                    variable: var.clone(),
                }
                .into()
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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        local_variables.get(&self.ident).map_or_else(
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
        )
    }
}

impl ReturnType for Ident {
    fn return_type(&self) -> crate::variable::Type {
        self.variable.as_type()
    }
}
