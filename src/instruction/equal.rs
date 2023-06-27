use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{error::Error, interpreter::VariableMap, parse::Rule};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Equal {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
}

impl Equal {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(variables, pair, local_variables)?.into();
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(variables, pair, local_variables)?.into();
        Ok(Self { lhs, rhs })
    }
}

impl Exec for Equal {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
        Ok((lhs == rhs).into())
    }
}

impl Recreate for Equal {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let lhs = self.lhs.recreate(local_variables, args).into();
        let rhs = self.rhs.recreate(local_variables, args).into();
        Instruction::Equal(Self { lhs, rhs })
    }
}

impl From<Equal> for Instruction {
    fn from(value: Equal) -> Self {
        Self::Equal(value)
    }
}
