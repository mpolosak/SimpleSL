use super::{ExecResult, Instruction, InstructionWithSpan, local_variable::LocalVariables};
use crate::{
    Error, Interpreter,
    errors::{CannotFieldAccess, NoField},
    instruction::{BaseInstruction, Exec, Recreate},
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct FieldAccess {
    instruction: Instruction,
    ident: Arc<str>,
}

impl FieldAccess {
    pub fn create_instruction(
        instruction: InstructionWithSpan,
        op: Pair<Rule>,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let return_type = instruction.return_type();
        if !return_type.is_struct() {
            let ins = instruction.span.as_str().into();
            errors.push(
                CannotFieldAccess {
                    ins,
                    var_type: return_type.clone(),
                }
                .into(),
            );
        }
        let pair = op.into_inner().next().unwrap();
        let ident = Arc::from(pair.as_str());

        if !return_type.has_field(&ident) {
            let struct_ident = instruction.span.as_str().into();
            errors.push(
                NoField {
                    struct_ident,
                    field_ident: ident.clone(),
                    struct_type: return_type,
                }
                .into(),
            );
        }
        Self {
            instruction: instruction.instruction,
            ident,
        }
        .into()
    }
}

impl BaseInstruction for FieldAccess {}

impl Exec for FieldAccess {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.instruction.exec(interpreter)?.into_struct().unwrap();
        Ok(var.get(&self.ident).unwrap().clone())
    }
}

impl Recreate for FieldAccess {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.instruction.recreate(local_variables);
        if let Some(var) = instruction.downcast_ref::<Variable>() {
            let s = var.clone().into_struct().unwrap();
            return s.get(&self.ident).unwrap().clone().into();
        }
        Self {
            instruction,
            ident: self.ident.clone(),
        }
        .into()
    }
}

impl ReturnType for FieldAccess {
    fn return_type(&self) -> Type {
        self.instruction
            .return_type()
            .field_type(&self.ident)
            .unwrap_or(Type::Never)
    }
}
