use super::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;
use std::result::Result;

#[derive(Clone, Debug)]
pub struct Multiply {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Multiply {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("*")),
        }
    }
}

impl Multiply {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(value1)),
                Instruction::Variable(Variable::Int(value2)),
            ) => Instruction::Variable((value1 * value2).into()),
            (
                Instruction::Variable(Variable::Float(value1)),
                Instruction::Variable(Variable::Float(value2)),
            ) => Instruction::Variable((value1 * value2).into()),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Multiply {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 * value2).into()),
            (Variable::Float(value1), Variable::Float(value2)) => Ok((value1 * value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Multiply {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl GetReturnType for Multiply {
    fn get_return_type(&self) -> Type {
        self.lhs.get_return_type()
    }
}

impl From<Multiply> for Instruction {
    fn from(value: Multiply) -> Self {
        Self::Multiply(value.into())
    }
}
