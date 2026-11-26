use crate::{
    errors::{ErrorWithRange, VariableDoesntExist},
    instruction::{
        BaseInstruction, Exec, Instruction, Recreate,
        local_variable::{LocalVariable, LocalVariables},
        parsing_context::ParsingContext,
    },
    variable::{ReturnType, Type, Typed},
};
use std::{ops::Range, sync::Arc};

#[derive(Debug)]
pub struct Ident {
    pub ident: Arc<str>,
    pub variable: LocalVariable,
}

impl Ident {
    pub fn create_instruction(
        range: Range<usize>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let ident = Arc::from(parsing_context.slice(range.clone()));
        parsing_context.local_variables.get(&ident).map_or_else(
            || {
                parsing_context
                    .local_variables
                    .interpreter
                    .get_variable(&ident)
                    .cloned()
                    .map(Instruction::from)
                    .unwrap_or_else(|| {
                        let error = VariableDoesntExist {
                            ident: ident.clone(),
                        }
                        .into();
                        errors.push(ErrorWithRange { error, range });
                        Self {
                            ident: ident.clone(),
                            variable: LocalVariable::Other(Type::Never),
                        }
                        .into()
                    })
            },
            |var| {
                Self {
                    ident: ident.clone(),
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
