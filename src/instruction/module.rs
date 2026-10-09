use super::{Instruction, local_variable::LocalVariables};
use crate::{
    Error,
    instruction::{
        block::Block, ident::Ident, local_variable::LocalVariableMap, set::Set, r#struct::Struct,
    },
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &LocalVariables,
) -> Result<Instruction, Error> {
    let mut local_variables = local_variables.create_layer();
    let instructions =
        local_variables.create_instructions(pair.into_inner().next().unwrap().into_inner())?;
    new(instructions, local_variables.drop_layer())
}

pub fn new(
    instructions: Box<[Instruction]>,
    lv_layer: LocalVariableMap,
) -> Result<Instruction, Error> {
    let sets = lv_layer
        .into_iter()
        .map(|(ident, variable)| Set::new_ident(ident.clone(), Ident { ident, variable }.into()))
        .collect();
    let struct_ins = Struct { sets }.into();
    let instructions = [instructions, [struct_ins].into()].concat().into();
    Ok(Block { instructions }.into())
}
