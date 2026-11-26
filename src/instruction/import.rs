use super::Instruction;
use crate::{
    errors::ErrorWithSpan,
    instruction::{module, panic::Panic, parsing_context::ParsingContext},
    source_span_from_pest_span,
    variable::from_str::string_from_pair,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let span = pair.as_span();
    let path = match string_from_pair(pair.into_inner().next().unwrap()) {
        Ok(path) => path,
        Err(error) => {
            let error = error;
            let span = source_span_from_pest_span(&span);
            errors.push(ErrorWithSpan { error, span });
            return Panic.into();
        }
    };
    let (instructions, layer) = match parsing_context.load(&path, errors) {
        Ok(i) => i,
        Err(error) => {
            let span = source_span_from_pest_span(&span);
            errors.push(ErrorWithSpan {
                error: error.into(),
                span,
            });
            return Panic {}.into();
        }
    };
    module::new(instructions, layer)
}
