use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Add {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) | (Type::String, Type::String) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (type1, type2) => Err(Error::CannotAdd(type1, type2)),
        }
    }
}

impl Add {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(value1)),
                Instruction::Variable(Variable::Int(value2)),
            ) => Instruction::Variable((value1 + value2).into()),
            (
                Instruction::Variable(Variable::Float(value1)),
                Instruction::Variable(Variable::Float(value2)),
            ) => Instruction::Variable((value1 + value2).into()),
            (
                Instruction::Variable(Variable::String(value1)),
                Instruction::Variable(Variable::String(value2)),
            ) => Instruction::Variable(format!("{value1}{value2}").into()),
            (rhs, lhs) => Self { rhs, lhs }.into(),
        }
    }
}

impl Exec for Add {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 + value2).into()),
            (Variable::Float(value1), Variable::Float(value2)) => Ok((value1 + value2).into()),
            (Variable::String(value1), Variable::String(value2)) => {
                Ok(format!("{value1}{value2}").into())
            }
            _ => panic!(),
        }
    }
}

impl Recreate for Add {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl GetReturnType for Add {
    fn get_return_type(&self) -> Type {
        self.lhs.get_return_type()
    }
}

impl From<Add> for Instruction {
    fn from(value: Add) -> Self {
        Self::Add(value.into())
    }
}
