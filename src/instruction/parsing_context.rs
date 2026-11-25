use std::{
    any::{Any, TypeId},
    fs,
};

use miette::SourceSpan;
use pest::{Parser, iterators::Pairs};
use simplesl_parser::{Rule, SimpleSLParser};

use crate::{
    Error,
    instruction::{
        Instruction,
        local_variable::{FunctionInfo, LocalVariableMap, LocalVariables},
    },
    variable::Variable,
};

pub struct ParsingContext<'a> {
    pub source: &'a str,
    pub local_variables: LocalVariables<'a>,
}

impl<'a> ParsingContext<'a> {
    pub fn create_layer(&'a self) -> Self {
        Self {
            source: self.source,
            local_variables: self.local_variables.create_layer(),
        }
    }

    pub fn function_layer(&'a self, layer: LocalVariableMap, function: FunctionInfo) -> Self {
        let local_variables = self.local_variables.function_layer(layer, function);
        Self {
            source: self.source,
            local_variables,
        }
    }

    pub(crate) fn load(&mut self, path: &str, errors: &mut Vec<Error>) -> Box<[Instruction]> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) => {
                errors.push(error.into());
                return [].into();
            }
        };
        self.parse_input(&contents, errors)
    }

    pub(crate) fn parse_input(
        &mut self,
        input: &str,
        errors: &mut Vec<Error>,
    ) -> Box<[Instruction]> {
        let pairs = match SimpleSLParser::parse(Rule::input, input) {
            Ok(pairs) => pairs,
            Err(error) => {
                errors.push(error.into());
                return [].into();
            }
        };
        self.create_instructions(pairs, errors)
    }

    pub(crate) fn create_instructions(
        &mut self,
        pairs: Pairs<'_, Rule>,
        errors: &mut Vec<Error>,
    ) -> Box<[Instruction]> {
        let mut instructions = pairs
            .map(|pair| Instruction::new(pair, self, errors))
            .collect::<Vec<Instruction>>();
        let Some(last) = instructions.pop() else {
            return Box::from([]);
        };
        instructions.retain(|instruction| instruction.0.type_id() != TypeId::of::<Variable>());
        instructions.push(last);
        instructions.into()
    }

    pub(crate) fn slice(&self, span: SourceSpan) -> &str {
        let start = span.offset();
        let end = start + span.len();
        &self.source[start..end]
    }
}
