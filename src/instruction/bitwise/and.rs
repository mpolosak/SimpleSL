use crate::instruction::traits::{BaseInstruction, BinIntOp, BinOp, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::interpreter::Interpreter;
use crate::variable::{Type, Typed, Variable};
use crate::Result;

#[derive(Debug)]
pub struct BitwiseAnd {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for BitwiseAnd {
    const SYMBOL: &'static str = "&";

    fn lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl BinIntOp for BitwiseAnd {}

impl CreateFromInstructions for BitwiseAnd {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::bin_and(lhs, rhs).into()
            }
            (lhs, rhs) => Self::construct(lhs, rhs).into(),
        })
    }
}

impl BitwiseAnd {
    fn bin_and(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs & rhs).into(),
            (var @ Variable::Array(_), _) | (_, var @ Variable::Array(_))
                if var.as_type() == Type::EmptyArray =>
            {
                var
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::bin_and(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for BitwiseAnd {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::bin_and(lhs, rhs))
    }
}

impl BaseInstruction for BitwiseAnd {}
