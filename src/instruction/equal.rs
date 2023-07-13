use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{error::Error, interpreter::VariableMap, parse::Rule};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Equal {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
}

impl Equal {
    pub fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        Ok(match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Instruction::Variable((variable == variable2).into())
            }
            (lhs, rhs) => Self {
                lhs: lhs.into(),
                rhs: rhs.into(),
            }
            .into(),
        })
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
