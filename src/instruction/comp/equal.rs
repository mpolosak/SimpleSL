use crate::instruction::traits::BinOp;
use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{interpreter::Interpreter, parse::Rule, variable::Variable, Result};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Equal {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Equal {
    const SYMBOL: &'static str = "==";

    fn get_lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn get_rhs(&self) -> &Instruction {
        &self.rhs
    }
}

impl CreateInstruction for Equal {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok((lhs == rhs).into())
    }
}

impl Recreate for Equal {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl From<Equal> for Instruction {
    fn from(value: Equal) -> Self {
        Self::Equal(value.into())
    }
}
