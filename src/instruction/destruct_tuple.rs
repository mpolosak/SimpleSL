use super::{
    local_variable::LocalVariables, traits::ExecResult, tuple::Tuple, Exec, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::{iter::zip, sync::Arc};

#[derive(Debug)]
pub struct DestructTuple {
    idents: Arc<[Arc<str>]>,
    instruction: InstructionWithStr,
}

impl DestructTuple {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let idents: Arc<[Arc<str>]> = pair.into_inner().map(|pair| pair.as_str().into()).collect();
        let pair = inner.next().unwrap();
        let instruction = InstructionWithStr::new(pair, local_variables)?;
        let return_type = instruction.return_type();
        if !return_type.is_tuple() {
            return Err(Error::NotATuple(instruction.str));
        }
        let Some(len) = return_type.tuple_len() else {
            return Err(Error::CannotDetermineLength(instruction.str));
        };
        let idents_len = idents.len();
        if len != idents_len {
            return Err(Error::WrongLength {
                ins: instruction.str,
                len,
                idents_len,
            });
        }
        let result = Self {
            idents,
            instruction,
        };
        result.insert_local_variables(local_variables);
        Ok(result.into())
    }
    fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        match &self.instruction.instruction {
            Instruction::Variable(Variable::Tuple(elements)) => {
                local_variables.extend(zip(self.idents.iter().cloned(), elements.iter().cloned()))
            }
            Instruction::Tuple(Tuple { elements }) => local_variables.extend(zip(
                self.idents.iter().cloned(),
                elements.iter().map(|ins| &ins.instruction),
            )),
            instruction => {
                let types = instruction.return_type().flatten_tuple().unwrap();
                local_variables.extend(zip(self.idents.iter().cloned(), types.iter().cloned()))
            }
        }
    }
}

impl Exec for DestructTuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = self.instruction.exec(interpreter)?.into_tuple().unwrap();
        for (ident, element) in zip(self.idents.iter().cloned(), elements.iter().cloned()) {
            interpreter.insert(ident, element);
        }
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for DestructTuple {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        let result = Self {
            idents: self.idents.clone(),
            instruction,
        };
        result.insert_local_variables(local_variables);
        Ok(result.into())
    }
}

impl ReturnType for DestructTuple {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
