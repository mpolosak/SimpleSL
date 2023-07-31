use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Modulo {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Modulo {
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
            (Type::Int, Type::Int) => Self::create_from_instructions(lhs, rhs),
            _ => Err(Error::BothOperandsMustBeInt("%")),
        }
    }
}
impl Modulo {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        match (lhs, rhs) {
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroModulo),
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Ok(Instruction::Variable((lhs % rhs).into())),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }
}

impl Exec for Modulo {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<crate::variable::Variable, Error> {
        let result1 = self.lhs.exec(interpreter)?;
        let result2 = self.rhs.exec(interpreter)?;
        match (result1, result2) {
            (Variable::Int(_), Variable::Int(0)) => Err(Error::ZeroModulo),
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 % value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Modulo {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(lhs, rhs)
    }
}

impl From<Modulo> for Instruction {
    fn from(value: Modulo) -> Self {
        Self::Modulo(value.into())
    }
}
