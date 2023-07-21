use crate::instruction::{
    local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{error::Error, interpreter::VariableMap, parse::Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Equal {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Equal {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl Equal {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Instruction::Variable((variable == variable2).into())
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
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
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let lhs = self.lhs.recreate(local_variables, args)?;
        let rhs = self.rhs.recreate(local_variables, args)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl From<Equal> for Instruction {
    fn from(value: Equal) -> Self {
        Self::Equal(value.into())
    }
}
