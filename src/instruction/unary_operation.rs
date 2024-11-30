use super::{
    at,
    function::call,
    local_variable::LocalVariables,
    prefix_op::{indirection, not, unary_minus},
    reduce::{all, any, bitand, bitor, product, sum},
    type_filter::TypeFilter,
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    variable::{ReturnType, Type},
    Error, Interpreter,
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
            Rule::at => at::create(lhs, op, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs, op.into_inner().next().unwrap())
            }
            Rule::function_call => call::create_instruction(lhs, op, local_variables),
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
pub struct UnaryOperation {
    pub instruction: Instruction,
    pub op: UnaryOperator,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    All,
    Any,
    BitAnd,
    BitOr,
    Sum,
    Product,
    Not,
    UnaryMinus,
    Indirection,
}

impl Exec for UnaryOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = interpreter.exec(&self.instruction)?;
        Ok(match self.op {
            UnaryOperator::All => all::exec(var),
            UnaryOperator::Any => any::exec(var),
            UnaryOperator::BitAnd => bitand::exec(var),
            UnaryOperator::BitOr => bitor::exec(var),
            UnaryOperator::Sum => sum::exec(var),
            UnaryOperator::Product => product::exec(var),
            UnaryOperator::Not => not::exec(var),
            UnaryOperator::UnaryMinus => unary_minus::exec(var),
            UnaryOperator::Indirection => indirection::exec(var),
        })
    }
}

impl Recreate for UnaryOperation {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
    ) -> Result<super::Instruction, crate::ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(match self.op {
            UnaryOperator::All => all::recreate(instruction),
            UnaryOperator::Any => any::recreate(instruction),
            UnaryOperator::BitAnd => bitand::recreate(instruction),
            UnaryOperator::BitOr => bitor::recreate(instruction),
            UnaryOperator::Sum => sum::recreate(instruction),
            UnaryOperator::Product => product::recreate(instruction),
            UnaryOperator::Not => not::create_from_instruction(instruction),
            UnaryOperator::UnaryMinus => unary_minus::create_from_instruction(instruction),
            op @ UnaryOperator::Indirection => UnaryOperation { instruction, op }.into(),
        })
    }
}

impl ReturnType for UnaryOperation {
    fn return_type(&self) -> Type {
        let return_type = self.instruction.return_type();
        match self.op {
            UnaryOperator::All | UnaryOperator::Any => Type::Bool,
            UnaryOperator::BitAnd | UnaryOperator::BitOr => Type::Int,
            UnaryOperator::Sum | UnaryOperator::Product => return_type.element_type().unwrap(),
            UnaryOperator::Not | UnaryOperator::UnaryMinus => return_type,
            UnaryOperator::Indirection => indirection::return_type(return_type),
        }
    }
}
