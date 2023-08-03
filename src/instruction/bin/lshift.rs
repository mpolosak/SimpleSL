use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct LShift {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for LShift {
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
            _ => Err(Error::BothOperandsMustBeInt("<<")),
        }
    }
}

impl LShift {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        match (lhs, rhs) {
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(Error::OverflowShift)
            }
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Ok(Instruction::Variable((lhs << rhs).into())),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }
}

impl Exec for LShift {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<crate::variable::Variable, Error> {
        let result1 = self.lhs.exec(interpreter)?;
        let result2 = self.rhs.exec(interpreter)?;
        match (result1, result2) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(Error::OverflowShift),
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 << value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for LShift {
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

impl From<LShift> for Instruction {
    fn from(value: LShift) -> Self {
        Self::LShift(value.into())
    }
}
