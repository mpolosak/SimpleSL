use std::rc::Rc;

use super::can_be_used;
use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::variable::{GetReturnType, GetType};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct RShift {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for RShift {
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
            Self::create_from_instructions(lhs, rhs)
        } else {
            Err(Error::CannotDo2(
                lhs.get_return_type(),
                ">>",
                rhs.get_return_type(),
            ))
        }
    }
}

impl RShift {
    fn rshift(lhs: Variable, rhs: Variable) -> Result<Variable> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(Error::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs >> rhs).into()),
            (array, _) | (_, array) if array.get_type().as_ref() == &Type::EmptyArray => Ok(array),
            (value, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|element| Self::rshift(value.clone(), element))
                .collect(),
            (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::rshift(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} >> {rhs} which is imposible"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::rshift(lhs, rhs)?.into())
            }
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(Error::OverflowShift)
            }
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }
}

impl Exec for RShift {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Self::rshift(lhs, rhs)
    }
}

impl Recreate for RShift {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(lhs, rhs)
    }
}

impl GetReturnType for RShift {
    fn get_return_type(&self) -> Rc<Type> {
        match (self.lhs.get_return_type(), self.rhs.get_return_type()) {
            (var_type, _) | (_, var_type)
                if matches!(var_type.as_ref(), Type::Array(_) | Type::EmptyArray) =>
            {
                var_type
            }
            (var_type, _) => var_type,
        }
    }
}

impl From<RShift> for Instruction {
    fn from(value: RShift) -> Self {
        Self::RShift(value.into())
    }
}
