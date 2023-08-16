use super::BitwiseBinOp;
use crate::instruction::traits::{BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction};
use crate::variable::GetReturnType;
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct BitwiseOr {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for BitwiseOr {
    const SYMBOL: &'static str = "|";

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

impl BitwiseBinOp for BitwiseOr {}

impl CreateInstruction for BitwiseOr {
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
        let lhs_type = lhs.get_return_type();
        let rhs_type = rhs.get_return_type();
        if Self::can_be_used(&lhs_type, &rhs_type) {
            Self::create_from_instructions(lhs, rhs)
        } else {
            Err(Error::CannotDo2(lhs_type, Self::SYMBOL, rhs_type))
        }
    }
}

impl CreateFromInstructions for BitwiseOr {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::bin_or(lhs, rhs).into()
            }
            (lhs, rhs) => Self::construct(lhs, rhs).into(),
        })
    }
}

impl BitwiseOr {
    fn bin_or(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs | rhs).into(),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (value, Variable::Array(array, _)) | (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::bin_or(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for BitwiseOr {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::bin_or(lhs, rhs))
    }
}

impl From<BitwiseOr> for Instruction {
    fn from(value: BitwiseOr) -> Self {
        Self::BinOr(value.into())
    }
}
