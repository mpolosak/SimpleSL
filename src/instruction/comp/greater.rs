use super::can_be_used;
use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::variable::{GetReturnType, Type};
use crate::{interpreter::Interpreter, parse::Rule, variable::Variable, Error, Result};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Greater {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Greater {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
        match (can_be_used(&lhs, &rhs), rule) {
            (true, Rule::greater) => Ok(Self::create_from_instructions(lhs, rhs)),
            (true, Rule::lower_equal) => Ok(Self::create_from_instructions(rhs, lhs)),
            (_, Rule::greater_equal) => Err(Error::OperandsMustBeBothIntOrBothFloat(">")),
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("<")),
        }
    }
}

impl Greater {
    fn greater(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs > rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs > rhs).into(),
            (lhs, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array, _), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::greater(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} > {rhs}"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::greater(lhs, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Greater {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::greater(lhs, rhs))
    }
}
impl Recreate for Greater {
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

impl GetReturnType for Greater {
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

impl From<Greater> for Instruction {
    fn from(value: Greater) -> Self {
        Self::Greater(value.into())
    }
}
