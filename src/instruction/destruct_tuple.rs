use super::{
    local_variable::LocalVariables, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{Typed, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{iter::zip, sync::Arc};

#[derive(Debug)]
pub struct DestructTuple {
    idents: Arc<[Arc<str>]>,
}

impl DestructTuple {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Arc<[Arc<str>]> = pair.into_inner().map(|pair| pair.as_str().into()).collect();
        let pair = inner.next().unwrap();
        let tuple_str = pair.as_str().into();
        InstructionWithStr::create(pair, local_variables, instructions)?;
        let return_type = local_variables.result.as_ref().unwrap().as_type();
        if !return_type.is_tuple() {
            return Err(Error::NotATuple(tuple_str));
        }
        let Some(len) = return_type.tuple_len() else {
            return Err(Error::CannotDetermineLength(tuple_str));
        };
        let idents_len = idents.len();
        if len != idents_len {
            return Err(Error::WrongLength {
                ins: tuple_str,
                len,
                idents_len,
            });
        }

        let types = return_type.flatten_tuple().unwrap();
        local_variables.extend(zip(idents.iter().cloned(), types.iter().cloned()));

        let instruction = Self { idents }.into();
        instructions.push(InstructionWithStr { instruction, str });
        Ok(())
    }
}

impl Exec for DestructTuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.result().unwrap().clone().into_tuple().unwrap();
        for (ident, element) in zip(self.idents.iter().cloned(), elements.iter().cloned()) {
            interpreter.insert(ident, element);
        }
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(&self, _local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        Ok(Self {
            idents: self.idents.clone(),
        }
        .into())
    }
}
