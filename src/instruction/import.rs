use super::{Instruction, local_variable::LocalVariables};
use crate::{Error, instruction::module, variable::Variable};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let mut local_variables = local_variables.create_layer();
    let path = Variable::try_from(pair.into_inner().next().unwrap())?
        .into_string()
        .unwrap();
    let instructions = local_variables.load(&path)?;
    module::new(instructions, local_variables.drop_layer())
}
