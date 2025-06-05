use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{Error, ExecError, Interpreter, variable::ReturnType};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FieldAccess {
    var: InstructionWithStr,
    ident: Arc<str>,
}

impl FieldAccess {
    pub fn create_instruction(
        tuple: InstructionWithStr,
        op: Pair<Rule>,
    ) -> Result<Instruction, Error> {
        let return_type = tuple.return_type();
        if !return_type.is_struct() {
            return Err(Error::CannotFieldAccess(tuple.str, return_type));
        }
        let pair = op.into_inner().next().unwrap();
        let ident = pair.as_str();
        if !return_type.has_field(ident) {
            return Err(Error::NoField {
                struct_ident: tuple.str,
                field_ident: ident.into(),
                struct_type: return_type,
            });
        }
        Ok(Self {
            var: tuple,
            ident: ident.into(),
        }
        .into())
    }
}

impl ReturnType for FieldAccess {
    fn return_type(&self) -> crate::variable::Type {
        self.var.return_type().field_type(&self.ident).unwrap()
    }
}

impl Recreate for FieldAccess {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let tuple = self.var.recreate(local_variables)?;
        let ident = self.ident.clone();
        Ok(Self { var: tuple, ident }.into())
    }
}

impl Exec for FieldAccess {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.var.exec(interpreter)?.into_struct().unwrap();
        Ok(var.get(&self.ident).unwrap().clone())
    }
}
