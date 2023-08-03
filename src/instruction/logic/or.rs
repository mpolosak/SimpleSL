use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Or {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Or {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) => Ok(Self::create_from_instructions(lhs, rhs)),
            _ => Err(Error::BothOperandsMustBeInt("||")),
        }
    }
}

impl Or {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Instruction::Variable((lhs != 0 || rhs != 0).into()),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value == 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Or {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let result1 = self.lhs.exec(interpreter)?;
        let result2 = self.rhs.exec(interpreter)?;
        match (result1, result2) {
            (Variable::Int(value1), Variable::Int(value2)) => {
                Ok((value1 != 0 || value2 != 0).into())
            }
            _ => panic!(),
        }
    }
}

impl Recreate for Or {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl From<Or> for Instruction {
    fn from(value: Or) -> Self {
        Self::Or(value.into())
    }
}
