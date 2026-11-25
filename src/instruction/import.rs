use super::Instruction;
use crate::{
    Error,
    instruction::{module, panic::Panic, parsing_context::ParsingContext},
    variable::from_str::string_from_pair,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<Error>,
) -> Instruction {
    let mut parsing_context = parsing_context.create_layer();
    let path = match string_from_pair(pair.into_inner().next().unwrap()) {
        Ok(path) => path,
        Err(error) => {
            errors.push(error);
            return Panic.into();
        }
    };
    let instructions = parsing_context.load(&path, errors);
    module::new(instructions, parsing_context.local_variables.drop_layer())
}
