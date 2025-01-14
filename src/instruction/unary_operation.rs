mod iter;
use super::{
    at,
    function::call,
    local_variable::LocalVariables,
    prefix_op::{indirection, not, unary_minus},
    reduce::{self, bool_reduce, collect, product, sum},
    tuple_access::TupleAccess,
    type_filter::TypeFilter,
    Exec, ExecResult, ExecStop, Instruction, InstructionWithStr, Recreate,
};
use crate::{
    unary_operator::UnaryOperator,
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
            Rule::tuple_access => TupleAccess::create_instruction(lhs, op),
            Rule::sum => sum::create(lhs),
            Rule::product => product::create(lhs),
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All),
            Rule::reduce_any => bool_reduce::create(lhs, UnaryOperator::Any),
            Rule::bitand_reduce => reduce::bit::create(lhs, UnaryOperator::BitAnd),
            Rule::bitor_reduce => reduce::bit::create(lhs, UnaryOperator::BitOr),
            Rule::collect => collect::create(lhs),
            Rule::iter => iter::create(lhs),
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

impl Exec for UnaryOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.instruction.exec(interpreter)?;
        Ok(match self.op {
            UnaryOperator::Sum => sum::exec(var)?,
            UnaryOperator::Product => product::exec(var)?,
            UnaryOperator::Not => not::exec(var),
            UnaryOperator::UnaryMinus => unary_minus::exec(var),
            UnaryOperator::Return => return Err(ExecStop::Return(var)),
            UnaryOperator::Indirection => indirection::exec(var),
            UnaryOperator::FunctionCall => var.into_function().unwrap().exec(interpreter)?,
            UnaryOperator::Collect => collect::exec(var, interpreter)?,
            UnaryOperator::Iter => iter::exec(var),
            UnaryOperator::All
            | UnaryOperator::Any
            | UnaryOperator::BitAnd
            | UnaryOperator::BitOr => unreachable!(),
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
            UnaryOperator::Not => not::create_from_instruction(instruction),
            UnaryOperator::UnaryMinus => unary_minus::create_from_instruction(instruction),
            op => UnaryOperation { instruction, op }.into(),
        })
    }
}

impl ReturnType for UnaryOperation {
    fn return_type(&self) -> Type {
        let return_type = self.instruction.return_type();
        match self.op {
            UnaryOperator::Sum | UnaryOperator::Product => return_type.iter_element().unwrap(),
            UnaryOperator::Not | UnaryOperator::UnaryMinus => return_type,
            UnaryOperator::Indirection => indirection::return_type(return_type),
            UnaryOperator::FunctionCall => return_type.return_type().unwrap(),
            UnaryOperator::Collect => collect::return_type(return_type),
            UnaryOperator::Iter => iter::return_type(return_type),
            UnaryOperator::All
            | UnaryOperator::Any
            | UnaryOperator::BitAnd
            | UnaryOperator::BitOr
            | UnaryOperator::Return => Type::Never,
        }
    }
}
