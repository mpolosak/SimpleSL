use crate::instruction::{
    local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::interpreter::Interpreter;
use crate::variable::Variable;
use crate::{
    error::Error,
    parse::Rule,
    variable::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct BinAnd {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for BinAnd {
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
            (Type::Int, Type::Int) => Ok(Self::create_from_instructions(lhs, rhs)),
            _ => Err(Error::BothOperandsMustBeInt("&")),
        }
    }
}
impl BinAnd {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Instruction::Variable((lhs & rhs).into()),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for BinAnd {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let result1 = self.lhs.exec(interpreter)?;
        let result2 = self.rhs.exec(interpreter)?;
        match (result1, result2) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 & value2).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for BinAnd {
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

impl From<BinAnd> for Instruction {
    fn from(value: BinAnd) -> Self {
        Self::BinAnd(value.into())
    }
}
