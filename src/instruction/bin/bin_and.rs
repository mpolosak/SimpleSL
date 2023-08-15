use super::can_be_used;
use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::interpreter::Interpreter;
use crate::variable::{GetReturnType, Variable};
use crate::{parse::Rule, variable::Type, Error, Result};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct BinAnd {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for BinAnd {
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
        if can_be_used(&lhs, &rhs) {
            Ok(Self::create_from_instructions(lhs, rhs))
        } else {
            Err(Error::CannotDo2(
                lhs.get_return_type(),
                "&",
                rhs.get_return_type(),
            ))
        }
    }
}
impl BinAnd {
    fn bin_and(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs & rhs).into(),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (value, Variable::Array(array, _)) | (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::bin_and(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} & {rhs} which is imposible"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::bin_and(lhs, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for BinAnd {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::bin_and(lhs, rhs))
    }
}

impl Recreate for BinAnd {
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

impl GetReturnType for BinAnd {
    fn get_return_type(&self) -> Type {
        if matches!(
            (self.lhs.get_return_type(), self.rhs.get_return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl From<BinAnd> for Instruction {
    fn from(value: BinAnd) -> Self {
        Self::BinAnd(value.into())
    }
}
