use super::{
    local_variable::LocalVariables, InstructionWithStr,
};
use crate::{
    variable::Variable,
    Error,
};
use pest::{iterators::Pair, Parser};
use simplesl_parser::{Rule, SimpleSLParser};
use std::fs;

pub fn create(pair: Pair<Rule>, local_variables: &mut LocalVariables, instructions: &mut Vec<InstructionWithStr>) -> Result<(), Error>{ 
    let path = Variable::try_from(pair.into_inner().next().unwrap())?
            .into_string()
            .unwrap();
    let input = fs::read_to_string(path.as_ref())?;
    let pairs = SimpleSLParser::parse(Rule::input, &input)?;
    for pair in pairs {
        InstructionWithStr::create(pair, local_variables, instructions)?;
    }
    Ok(())
}
