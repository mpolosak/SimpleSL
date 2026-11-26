use super::Instruction;
use crate::{
    errors::ErrorWithSpan,
    instruction::{
        block::Block, ident::Ident, local_variable::LocalVariableMap,
        parsing_context::ParsingContext, set::Set, r#struct::Struct,
    },
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let mut parsing_context = parsing_context.create_layer();
    let instructions =
        parsing_context.create_instructions(pair.into_inner().next().unwrap().into_inner(), errors);
    new(instructions, parsing_context.local_variables.drop_layer())
}

pub fn new(instructions: Box<[Instruction]>, lv_layer: LocalVariableMap) -> Instruction {
    let sets = lv_layer
        .into_iter()
        .map(|(ident, variable)| Set::new_ident(ident.clone(), Ident { ident, variable }.into()))
        .collect();
    let struct_ins = Struct { sets }.into();
    let instructions = [instructions, [struct_ins].into()].concat().into();
    Block { instructions }.into()
}
