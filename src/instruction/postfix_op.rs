mod iter;
use super::{
    InstructionWithSpan, at,
    reduce::{self, bool_reduce, collect},
    tuple_access,
    type_filter::TypeFilter,
};
use crate::{
    Error,
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
use miette::SourceSpan;
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};

impl InstructionWithSpan {
    pub fn create_postfix(
        op: Pair<'_, Rule>,
        lhs: Self,
        parsing_context: &ParsingContext,
        errors: &mut Vec<Error>,
    ) -> Self {
        let start = lhs.span.offset();
        let end = op.as_span().end();
        let len = end - start;
        let span = SourceSpan::new(start.into(), len);
        let instruction = match op.as_rule() {
            Rule::at => at::create(lhs.instruction, op, parsing_context, errors),
            Rule::type_filter => TypeFilter::create_instruction(
                lhs.instruction,
                op.into_inner().next().unwrap(),
                errors,
            ),
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
        Self { instruction, span }
    }
}
