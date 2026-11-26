use crate::{
    errors::{ErrorWithRange, ErrorsWithSource, Source},
    instruction::{
        Instruction,
        local_variable::{FunctionInfo, LocalVariableMap, LocalVariables},
    },
    variable::Variable,
};
use pest::{Parser, iterators::Pairs};
use simplesl_parser::{Rule, SimpleSLParser};
use std::{
    any::{Any, TypeId},
    fs, io,
    ops::Range,
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

    pub(crate) fn load(
        &self,
        path: &str,
        errors: &mut Vec<ErrorWithRange>,
    ) -> io::Result<(Box<[Instruction]>, LocalVariableMap)> {
        let contents = fs::read_to_string(path)?;
        let mut errors2 = Vec::new();
        let res = self.parse_input(&contents, &mut errors2);
        if !errors2.is_empty() {
            let source = Source::new(path, contents);
            let error = ErrorsWithSource {
                source,
                errors: errors2,
            }
            .into();
            errors.push(ErrorWithRange { error, range: 0..0 });
        }
        Ok(res)
    }

    pub(crate) fn parse_input(
        &'a self,
        input: &'a str,
        errors: &mut Vec<ErrorWithRange>,
    ) -> (Box<[Instruction]>, LocalVariableMap) {
        let pairs = match SimpleSLParser::parse(Rule::input, input) {
            Ok(pairs) => pairs,
            Err(error) => {
                errors.push(error.into());
                return ([].into(), LocalVariableMap::new());
            }
        };
        let mut parsing_context = Self {
            source: input,
            local_variables: self.local_variables.create_layer(),
        };
        let instructions = parsing_context.create_instructions(pairs, errors);
        let layer = parsing_context.local_variables.drop_layer();
        (instructions, layer)
    }

    pub(crate) fn create_instructions(
        &mut self,
        pairs: Pairs<'_, Rule>,
        errors: &mut Vec<ErrorWithRange>,
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

    pub(crate) fn slice(&self, range: Range<usize>) -> &str {
        &self.source[range]
    }
}
