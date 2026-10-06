use super::{
    Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, ExecError,
    instruction::set::Set,
    interpreter::{Interpreter, VariableMap},
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Struct {
    pub sets: Arc<[Set]>,
}

impl Struct {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let sets = pair
            .into_inner()
            .map(|pair| {
                if pair.as_rule() == Rule::ident {
                    let ident: Arc<str> = pair.as_str().into();
                    let instruction = Instruction::new_ident(&ident, local_variables)?;
                    return Ok(Set::new_ident(ident, instruction));
                }
                let mut local_variables = local_variables.create_layer();
                Set::create_standalone(pair, &mut local_variables)
            })
            .collect::<Result<Arc<[Set]>, Error>>()?;
        Ok(Self { sets }.into())
    }
}

impl Exec for Struct {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut vm = VariableMap::new();
        for set in self.sets.iter() {
            let mut interpreter = interpreter.create_layer();
            set.exec(&mut interpreter)?;
            vm.extend(interpreter.drop_layer());
        }
        Ok(Variable::Struct(vm.into()))
    }
}

impl Recreate for Struct {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let sets = self
            .sets
            .iter()
            .map(|set| {
                let mut local_variables = local_variables.create_layer();
                set.inner_recreate(&mut local_variables)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { sets }.into())
    }
}

impl ReturnType for Struct {
    fn return_type(&self) -> Type {
        let mut tm = HashMap::new();
        for set in self.sets.iter() {
            set.pattern.insert_types(&mut tm);
        }
        Type::Struct(tm.into())
    }
}
