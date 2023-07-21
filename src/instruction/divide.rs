use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Divide {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Divide {
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
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                Self::create_from_instructions(lhs, rhs)
            }
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("/")),
        }
    }
}

impl Divide {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        match (lhs, rhs) {
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroDivision),
            (
                Instruction::Variable(Variable::Int(value1)),
                Instruction::Variable(Variable::Int(value2)),
            ) => Ok(Instruction::Variable((value1 / value2).into())),
            (
                Instruction::Variable(Variable::Float(value1)),
                Instruction::Variable(Variable::Float(value2)),
            ) => Ok(Instruction::Variable((value1 / value2).into())),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }
}

impl Exec for Divide {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
        match (lhs, rhs) {
            (Variable::Int(_), Variable::Int(0)) => Err(Error::ZeroDivision),
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 / value2).into()),
            (Variable::Float(value1), Variable::Float(value2)) => Ok((value1 / value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Divide {
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let lhs = self.lhs.recreate(local_variables, args)?;
        let rhs = self.rhs.recreate(local_variables, args)?;
        Self::create_from_instructions(lhs, rhs)
    }
}

impl GetReturnType for Divide {
    fn get_return_type(&self) -> Type {
        self.lhs.get_return_type()
    }
}

impl From<Divide> for Instruction {
    fn from(value: Divide) -> Self {
        Self::Divide(value.into())
    }
}
