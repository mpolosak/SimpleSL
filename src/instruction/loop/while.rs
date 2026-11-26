use crate::{
    errors::ErrorWithRange,
    instruction::{
        Instruction, Loop, control_flow::If, r#loop::Break, parsing_context::ParsingContext,
    },
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &mut ParsingContext,
    errors: &mut Vec<ErrorWithRange>,
) -> Instruction {
    let in_loop = parsing_context.local_variables.in_loop;
    parsing_context.local_variables.in_loop = true;
    let mut if_ins = If::create(pair, parsing_context, errors);
    parsing_context.local_variables.in_loop = in_loop;
    if_ins.else_instruction = Break.into();
    Loop(if_ins.into()).into()
}
