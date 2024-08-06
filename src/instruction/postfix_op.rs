use super::{
    at::At,
    local_variable::LocalVariables,
    reduce::{all, any, bitand, bitor, product, sum},
    type_filter::TypeFilter,
    Exec, FunctionCall, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    variable::{ReturnType, Type},
    Error,
};
use pest::iterators::Pair;
use simplesl_parser::{unexpected, Rule};

impl InstructionWithStr {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let str = format!("{} {}", lhs.str, op.as_str()).into();
        let instruction = match op.as_rule() {
            Rule::at => At::create_instruction(lhs, op, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs, op.into_inner().next().unwrap())
            }
            Rule::function_call => FunctionCall::create_instruction(lhs, op, local_variables),
            Rule::sum => sum::create(lhs),
            Rule::product => product::create(lhs),
            Rule::all => all::create(lhs),
            Rule::reduce_any => any::create(lhs),
            Rule::bitand_reduce => bitand::create(lhs),
            Rule::bitor_reduce => bitor::create(lhs),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}

#[derive(Debug)]
pub struct PostfixOperation {
    pub instruction: InstructionWithStr,
    pub op: PostfixOperator,
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixOperator {
    All,
    Any,
    BitAnd,
    BitOr,
    Sum,
    Product,
}

impl Exec for PostfixOperation {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> super::ExecResult {
        let var = self.instruction.exec(interpreter)?;
        match self.op {
            PostfixOperator::All => all::exec(var),
            PostfixOperator::Any => any::exec(var),
            PostfixOperator::BitAnd => bitand::exec(var),
            PostfixOperator::BitOr => bitor::exec(var),
            PostfixOperator::Sum => sum::exec(var),
            PostfixOperator::Product => product::exec(var),
        }
    }
}

impl Recreate for PostfixOperation {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
    ) -> Result<super::Instruction, crate::ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        match self.op {
            PostfixOperator::All => all::recreate(instruction),
            PostfixOperator::Any => any::recreate(instruction),
            PostfixOperator::BitAnd => bitand::recreate(instruction),
            PostfixOperator::BitOr => bitor::recreate(instruction),
            PostfixOperator::Sum => sum::recreate(instruction),
            PostfixOperator::Product => product::recreate(instruction),
        }
    }
}

impl ReturnType for PostfixOperation {
    fn return_type(&self) -> Type {
        let return_type = self.instruction.return_type();
        match self.op {
            PostfixOperator::All
            | PostfixOperator::Any
            | PostfixOperator::BitAnd
            | PostfixOperator::BitOr => Type::Int,
            PostfixOperator::Sum | PostfixOperator::Product => return_type.element_type().unwrap(),
        }
    }
}

impl From<PostfixOperation> for Instruction {
    fn from(value: PostfixOperation) -> Self {
        Self::PostfixOperation(value.into())
    }
}
