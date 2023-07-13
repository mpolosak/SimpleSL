use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Xor {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
}

impl CreateInstruction for Xor {
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
            (Type::Int, Type::Int) => Ok(Self::create_from_instructions(lhs, rhs)),
            _ => Err(Error::BothOperandsMustBeInt("^")),
        }
    }
}
impl Xor {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Instruction::Variable((lhs ^ rhs).into()),
            (lhs, rhs) => Self {
                lhs: lhs.into(),
                rhs: rhs.into(),
            }
            .into(),
        }
    }
}

impl Exec for Xor {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let result1 = self.lhs.exec(interpreter, local_variables)?;
        let result2 = self.rhs.exec(interpreter, local_variables)?;
        match (result1, result2) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 ^ value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Xor {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let lhs = self.lhs.recreate(local_variables, args);
        let rhs = self.rhs.recreate(local_variables, args);
        Self::create_from_instructions(lhs, rhs)
    }
}

impl From<Xor> for Instruction {
    fn from(value: Xor) -> Self {
        Self::Xor(value)
    }
}
