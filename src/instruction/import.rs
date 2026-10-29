use super::{Instruction, local_variable::LocalVariables};
use crate::{instruction::{module, panic::Panic}, variable::from_str::string_from_pair, Error};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
    errors: &mut Vec<Error>
) -> Instruction {
    let mut local_variables = local_variables.create_layer();
    let path = match string_from_pair(pair.into_inner().next().unwrap()){
        Ok(path) => path,
        Err(error) => {
            errors.push(error);
            return Panic.into();
        },
    };
    let instructions = local_variables.load(&path, errors);
    module::new(instructions, local_variables.drop_layer())
}
