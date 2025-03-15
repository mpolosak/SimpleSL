use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error, ExecError, Interpreter,
    variable::{ReturnType, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug, Clone)]
pub struct TupleAccess {
    tuple: InstructionWithStr,
    index: usize,
}

impl TupleAccess {
    pub fn create_instruction(
        tuple: InstructionWithStr,
        op: Pair<Rule>,
    ) -> Result<Instruction, Error> {
        let return_type = tuple.return_type();
        if !return_type.is_tuple() {
            return Err(Error::CannotTupleAccess(tuple.str, return_type));
        }
        let pair = op.into_inner().next().unwrap();
        let index = Variable::try_from(pair)?.into_int().unwrap() as usize;
        let len = return_type.min_tuple_len().unwrap();
        if index >= len {
            return Err(Error::TupleIndexTooBig(index, tuple.str, len));
        }
        Ok(Self { tuple, index }.into())
    }
}

impl ReturnType for TupleAccess {
    fn return_type(&self) -> crate::variable::Type {
        self.tuple
            .return_type()
            .tuple_element_at(self.index)
            .unwrap()
    }
}

impl Recreate for TupleAccess {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let tuple = self.tuple.recreate(local_variables)?;
        Ok(Self {
            tuple,
            index: self.index,
        }
        .into())
    }
}

impl Exec for TupleAccess {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let tuple = self.tuple.exec(interpreter)?.into_tuple().unwrap();
        Ok(tuple[self.index].clone())
    }
}
