mod iter;
use super::{
    InstructionWithStr, at,
    local_variable::LocalVariables,
    reduce::{self, bool_reduce, collect},
    tuple_access,
    type_filter::TypeFilter,
};
use crate::{
    instruction::{field_access::FieldAccess, function::call::FunctionCall, postfix_op::iter::Iter, reduce::{product::Product, sum::Sum}, slicing}, unary_operator::UnaryOperator, Error
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithStr {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        let str = format!("{} {}", lhs.str, op.as_str()).into();
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs.instruction, op, local_variables),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs.instruction, op.into_inner().next().unwrap())
            }
            Rule::function_call => FunctionCall::create_instruction(lhs, op, local_variables),
            Rule::tuple_access => tuple_access::create_instruction(lhs, op),
            Rule::field_access => FieldAccess::create_instruction(lhs, op),
            Rule::sum => Sum::create(lhs),
            Rule::product => Product::create(lhs),
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All),
            Rule::reduce_any => bool_reduce::create(lhs, UnaryOperator::Any),
            Rule::bitand_reduce => reduce::bit::create(lhs, UnaryOperator::BitAnd),
            Rule::bitor_reduce => reduce::bit::create(lhs, UnaryOperator::BitOr),
            Rule::collect => collect::create(lhs),
            Rule::iter => Iter::create_instruction(lhs),
            Rule::slicing => slicing::create(lhs, op, local_variables),
            rule => unexpected!(rule),
        }?;
        Ok(Self { instruction, str })
    }
}
