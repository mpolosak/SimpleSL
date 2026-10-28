use super::{ExecResult, Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    instruction::{
        BaseInstruction, Exec, Recreate,
    }, variable::{ReturnType, Type, Variable}, Error, Interpreter
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
        instruction: InstructionWithStr,
        op: Pair<Rule>,
        errors: &mut Vec<Error>
    ) -> Instruction {
        let return_type = instruction.return_type();
        if !return_type.is_struct() {
            errors.push(Error::CannotFieldAccess(instruction.str.clone(), return_type.clone()));
        }
        let pair = op.into_inner().next().unwrap();
        let ident = Arc::from(pair.as_str());

        if !return_type.has_field(&ident) {
            errors.push(Error::NoField {
                struct_ident: instruction.str,
                field_ident: ident.clone(),
                struct_type: return_type,
            });
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
