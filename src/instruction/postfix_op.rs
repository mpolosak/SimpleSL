mod iter;
use super::{
    InstructionWithRange, at,
    reduce::{self, bool_reduce, collect},
    tuple_access,
    type_filter::TypeFilter,
};
use crate::{
    errors::ErrorWithRange,
    instruction::{
        field_access::FieldAccess,
        function::call::FunctionCall,
        parsing_context::ParsingContext,
        postfix_op::iter::Iter,
        reduce::{product::Product, sum::Sum},
        slicing,
    },
    unary_operator::UnaryOperator,
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithRange {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Self {
        let range = lhs.range.start..op.as_span().end();
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs, op, parsing_context, errors),
            Rule::type_filter => {
                TypeFilter::create_instruction(lhs, op.into_inner().next().unwrap(), errors)
            }
            Rule::function_call => {
                FunctionCall::create_instruction(lhs, op, parsing_context, errors)
            }
            Rule::tuple_access => {
                tuple_access::create_instruction(lhs, op, parsing_context, errors)
            }
            Rule::field_access => FieldAccess::create_instruction(lhs, op, parsing_context, errors),
            Rule::sum => Sum::create(lhs, parsing_context, errors),
            Rule::product => Product::create(lhs, parsing_context, errors),
            Rule::all => bool_reduce::create(lhs, UnaryOperator::All, parsing_context, errors),
            Rule::reduce_any => {
                bool_reduce::create(lhs, UnaryOperator::Any, parsing_context, errors)
            }
            Rule::bitand_reduce => {
                reduce::bit::create(lhs, UnaryOperator::BitAnd, parsing_context, errors)
            }
            Rule::bitor_reduce => {
                reduce::bit::create(lhs, UnaryOperator::BitOr, parsing_context, errors)
            }
            Rule::collect => collect::create(lhs, parsing_context, errors),
            Rule::iter => Iter::create_instruction(lhs, parsing_context, errors),
            Rule::slicing => slicing::create(lhs, op, parsing_context, errors),
            rule => unexpected!(rule),
        };
        Self { instruction, range }
    }
}
