use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, ExecError,
    instruction::ExecStop,
    interpreter::{Interpreter, VariableMap},
    variable::{ReturnType, StructType, Type, Variable},
};
use itertools::Itertools;
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Struct(HashMap<Arc<str>, InstructionWithStr>);

impl Struct {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let fields = pair
            .into_inner()
            .tuples()
            .map(|(ident, value)| {
                let ident: Arc<str> = ident.as_str().into();
                let value = InstructionWithStr::new_expression(value, local_variables)?;
                Ok((ident, value))
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self(fields).into())
    }
}

impl Exec for Struct {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let vm = self
            .0
            .iter()
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
        let fields = self
            .0
            .iter()
            .map(|(ident, value)| {
                let value = value.recreate(local_variables)?;
                let ident = ident.clone();
                Ok((ident, value))
            })
            .collect::<Result<_, ExecError>>()?;
        Ok(Self(fields).into())
    }
}

impl ReturnType for Struct {
    fn return_type(&self) -> Type {
        let tm: HashMap<Arc<str>, Type> = self
            .0
            .iter()
            .map(|(key, value)| (key.clone(), value.return_type()))
            .collect();
        StructType(Arc::from(tm)).into()
    }
}
