use crate::instruction::traits::{BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction};
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

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for Equal {
    fn can_be_used(_lhs: &crate::variable::Type, _rhs: &crate::variable::Type) -> bool {
        true
    }
}

impl CreateFromInstructions for Equal {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Ok(Instruction::Variable((variable == variable2).into()))
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
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
        Self::create_from_instructions(lhs, rhs)
    }
}

impl Exec for Equal {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok((lhs == rhs).into())
    }
}

impl From<Equal> for Instruction {
    fn from(value: Equal) -> Self {
        Self::Equal(value.into())
    }
}
