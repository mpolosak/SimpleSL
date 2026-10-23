use crate::{
    Error, Interpreter,
    instruction::{
        Exec, ExecStop, Instruction,
        local_variable::LocalVariables,
        pattern::{Pattern, destruct_pattern::DestructPattern},
        recreate_instructions,
    },
    variable::{Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::{Rule, unexpected};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum MatchPattern {
    Pattern(Pattern),
    Values(Arc<[Instruction]>),
    Other,
}

impl MatchPattern {
    pub fn new(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        exp_type: &Type,
    ) -> Result<MatchPattern, Error> {
        let rule = pair.as_rule();
        Ok(match rule {
            Rule::pattern => Pattern::create_instruction(pair, exp_type)?.into(),
            Rule::values => {
                let inner_values = pair.into_inner();
                let values = inner_values
                    .map(|pair| Instruction::new(pair, local_variables))
                    .collect::<Result<Arc<[Instruction]>, Error>>()?;
                MatchPattern::Values(values)
            }
            Rule::other => MatchPattern::Other,
            _ => unexpected!(rule),
        })
    }

    pub fn is_covering_type(&self, checked_type: &Type) -> bool {
        match self {
            MatchPattern::Pattern(pattern) => {
                if matches!(&pattern.destruct_pattern, DestructPattern::Array(_)) {
                    false
                } else {
                    pattern.is_matched(checked_type)
                }
            }
            MatchPattern::Values(_) => false,
            MatchPattern::Other => true,
        }
    }

    pub fn covers(
        &self,
        variable: &Variable,
        interpreter: &mut Interpreter,
    ) -> Result<bool, ExecStop> {
        Ok(match self {
            MatchPattern::Pattern(pattern) => pattern.matches_variable(variable),
            MatchPattern::Values(instructions) => {
                for instruction in instructions.iter() {
                    let match_variable = instruction.exec(interpreter)?;
                    if match_variable == *variable {
                        return Ok(true);
                    }
                }
                false
            }
            MatchPattern::Other => true,
        })
    }

    pub fn recreate(&self, local_variables: &mut LocalVariables) -> Self {
        let MatchPattern::Values(instructions) = self else {
            return self.clone();
        };
        let instructions = recreate_instructions(instructions, local_variables).into();
        MatchPattern::Values(instructions)
    }
}

impl From<Pattern> for MatchPattern {
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}
