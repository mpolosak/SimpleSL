use super::{local_variable::LocalVariableMap, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct GreaterOrEqual {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
}

impl GreaterOrEqual {
    pub fn new(
        variables: &VariableMap,
        pair: Pair<Rule>,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(variables, pair, local_variables)?;
        let pair = inner.next().unwrap();
        let instruction2 = Instruction::new(variables, pair, local_variables)?;
        match (
            instruction.get_return_type(),
            instruction2.get_return_type(),
            rule,
        ) {
            (Type::Int, Type::Int, Rule::greater_equal)
            | (Type::Float, Type::Float, Rule::greater_equal) => Ok(Self {
                lhs: instruction.into(),
                rhs: instruction2.into(),
            }),
            (Type::Int, Type::Int, Rule::lower_equal)
            | (Type::Float, Type::Float, Rule::lower_equal) => Ok(Self {
                lhs: instruction2.into(),
                rhs: instruction.into(),
            }),
            (_, _, Rule::greater) => Err(Error::Other(String::from(
                "Arguments of >= operator must be both int or both float",
            ))),
            _ => Err(Error::Other(String::from(
                "Arguments of <= operator must be both int or both float",
            ))),
        }
    }
}

impl Exec for GreaterOrEqual {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs >= rhs).into()),
            (Variable::Float(lhs), Variable::Float(rhs)) => Ok((lhs >= rhs).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for GreaterOrEqual {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let lhs = self.lhs.recreate(local_variables, args).into();
        let rhs = self.rhs.recreate(local_variables, args).into();
        Self { lhs, rhs }.into()
    }
}

impl From<GreaterOrEqual> for Instruction {
    fn from(value: GreaterOrEqual) -> Self {
        Self::GreaterOrEqual(value)
    }
}
