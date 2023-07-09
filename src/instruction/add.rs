use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Add {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
}

impl Add {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) | (Type::String, Type::String) => {
                Ok(Self {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                })
            }
            (type1, type2) => Err(Error::CannotAdd(type1, type2)),
        }
    }
}

impl Exec for Add {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
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
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let lhs = self.lhs.recreate(local_variables, args).into();
        let rhs = self.rhs.recreate(local_variables, args).into();
        Instruction::Add(Self { lhs, rhs })
    }
}

impl GetReturnType for Add {
    fn get_return_type(&self) -> Type {
        self.lhs.get_return_type()
    }
}

impl From<Add> for Instruction {
    fn from(value: Add) -> Self {
        Self::Add(value)
    }
}
