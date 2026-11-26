use super::{ExecResult, Instruction, InstructionWithSpan, local_variable::LocalVariables};
use crate::{
    Interpreter,
    errors::{CannotFieldAccess, ErrorWithSpan, NoField},
    instruction::{BaseInstruction, Exec, Recreate, parsing_context::ParsingContext},
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
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let return_type = instruction.return_type();
        if !return_type.is_struct() {
            let span = instruction.span;
            let ins = parsing_context.slice(span).into();
            let error = CannotFieldAccess {
                ins,
                var_type: return_type.clone(),
            }
            .into();
            errors.push(ErrorWithSpan { error, span });
        }
        let pair = op.into_inner().next().unwrap();
        let ident = Arc::from(pair.as_str());

        if !return_type.has_field(&ident) {
            let span = instruction.span;
            let struct_ident = parsing_context.slice(span).into();
            let error = NoField {
                struct_ident,
                field_ident: ident.clone(),
                struct_type: return_type,
            }
            .into();
            errors.push(ErrorWithSpan { error, span });
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
