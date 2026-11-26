use super::Instruction;
use crate::{
    errors::ErrorWithRange,
    instruction::{module, panic::Panic, parsing_context::ParsingContext},
    range_from_pest_span,
    variable::from_str::string_from_pair,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithRange>,
) -> Instruction {
    let span = pair.as_span();
    let path = match string_from_pair(pair.into_inner().next().unwrap()) {
        Ok(path) => path,
        Err(error) => {
            let error = error;
            let span = range_from_pest_span(&span);
            errors.push(ErrorWithRange { error, range: span });
            return Panic.into();
        }
    };
    let (instructions, layer) = match parsing_context.load(&path, errors) {
        Ok(i) => i,
        Err(error) => {
            let span = range_from_pest_span(&span);
            errors.push(ErrorWithRange {
                error: error.into(),
                range: span,
            });
            return Panic {}.into();
        }
    };
    module::new(instructions, layer)
}
