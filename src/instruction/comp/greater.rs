use crate::instruction::{
    local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Greater {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Greater {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let instruction = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let instruction2 = Instruction::new(pair, variables, local_variables)?;
        match (
            instruction.get_return_type(),
            instruction2.get_return_type(),
            rule,
        ) {
            (Type::Int, Type::Int, Rule::greater) | (Type::Float, Type::Float, Rule::greater) => {
                Ok(Self::create_from_instructions(instruction, instruction2))
            }
            (Type::Int, Type::Int, Rule::lower) | (Type::Float, Type::Float, Rule::lower) => {
                Ok(Self::create_from_instructions(instruction2, instruction))
            }
            (_, _, Rule::greater) => Err(Error::OperandsMustBeBothIntOrBothFloat(">")),
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("<")),
        }
    }
}

impl Greater {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (
                Instruction::Variable(Variable::Int(lhs)),
                Instruction::Variable(Variable::Int(rhs)),
            ) => Instruction::Variable((lhs > rhs).into()),
            (
                Instruction::Variable(Variable::Float(lhs)),
                Instruction::Variable(Variable::Float(rhs)),
            ) => Instruction::Variable((lhs > rhs).into()),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Greater {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let lhs = self.lhs.exec(interpreter, local_variables)?;
        let rhs = self.rhs.exec(interpreter, local_variables)?;
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs > rhs).into()),
            (Variable::Float(lhs), Variable::Float(rhs)) => Ok((lhs > rhs).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Greater {
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

impl From<Greater> for Instruction {
    fn from(value: Greater) -> Self {
        Self::Greater(value.into())
    }
}
