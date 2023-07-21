use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Subtract {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Subtract {
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
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("-")),
        }
    }
}

impl Subtract {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(value1)),
                Instruction::Variable(Variable::Int(value2)),
            ) => Instruction::Variable((value1 - value2).into()),
            (
                Instruction::Variable(Variable::Float(value1)),
                Instruction::Variable(Variable::Float(value2)),
            ) => Instruction::Variable((value1 - value2).into()),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Subtract {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 - value2).into()),
            (Variable::Float(value1), Variable::Float(value2)) => Ok((value1 - value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Subtract {
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

impl GetReturnType for Subtract {
    fn get_return_type(&self) -> Type {
        self.lhs.get_return_type()
    }
}

impl From<Subtract> for Instruction {
    fn from(value: Subtract) -> Self {
        Self::Subtract(value.into())
    }
}
