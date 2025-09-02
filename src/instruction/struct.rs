use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, ExecError,
    instruction::ExecStop,
    interpreter::{Interpreter, VariableMap},
    variable::{ReturnType, StructType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Struct {
    pub idents: Arc<[Arc<str>]>,
    pub values: Arc<[InstructionWithStr]>,
}

impl Struct {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let (idents, values) = pair
            .into_inner()
            .map(|pair| {
                if pair.as_rule() == Rule::ident {
                    let ident: Arc<str> = pair.as_str().into();
                    let value = InstructionWithStr::new_ident(ident.clone(), local_variables)?;
                    return Ok((ident, value));
                }
                let mut inner = pair.into_inner();
                let ident: Arc<str> = inner.next().unwrap().as_str().into();
                let value =
                    InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
                Ok((ident, value))
            })
            .collect::<Result<(Vec<Arc<str>>, Vec<InstructionWithStr>), Error>>()?;
        Ok(Self {
            idents: idents.into(),
            values: values.into(),
        }
        .into())
    }
}

impl Exec for Struct {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let vm = self
            .idents
            .iter()
            .zip(self.values.iter())
            .map(|(ident, value)| {
                let value = value.exec(interpreter)?;
                let ident = ident.clone();
                Ok((ident, value))
            })
            .collect::<Result<VariableMap, ExecStop>>()?;
        Ok(Variable::Struct(vm.into()))
    }
}

impl Recreate for Struct {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let idents = self.idents.clone();
        let values = self
            .values
            .iter()
            .map(|value| value.recreate(local_variables))
            .collect::<Result<_, _>>()?;
        Ok(Self { idents, values }.into())
    }
}

impl ReturnType for Struct {
    fn return_type(&self) -> Type {
        let tm: HashMap<Arc<str>, Type> = self
            .idents
            .iter()
            .zip(self.values.iter())
            .map(|(key, value)| (key.clone(), value.return_type()))
            .collect();
        StructType(Arc::from(tm)).into()
    }
}
