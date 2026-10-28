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
        errors: &mut Vec<Error>
    ) -> Self {
        let str = format!("{} {}", lhs.str, op.as_str()).into();
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs.instruction, op, local_variables, errors),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs.instruction, op.into_inner().next().unwrap(), errors)
            }
            Rule::function_call => FunctionCall::create_instruction(lhs, op, local_variables, errors),
            Rule::tuple_access => tuple_access::create_instruction(lhs, op, errors),
            Rule::field_access => FieldAccess::create_instruction(lhs, op, errors),
            Rule::sum => Sum::create(lhs, errors),
            Rule::product => Product::create(lhs, errors),
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All, errors),
            Rule::reduce_any => bool_reduce::create(lhs, UnaryOperator::Any, errors),
            Rule::bitand_reduce => reduce::bit::create(lhs, UnaryOperator::BitAnd, errors),
            Rule::bitor_reduce => reduce::bit::create(lhs, UnaryOperator::BitOr, errors),
            Rule::collect => collect::create(lhs, errors),
            Rule::iter => Iter::create_instruction(lhs, errors),
            Rule::slicing => slicing::create(lhs, op, local_variables, errors),
            rule => unexpected!(rule),
        };
        Self { instruction, str }
    }
}
