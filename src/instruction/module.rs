use std::sync::Arc;

use super::{Instruction, local_variable::LocalVariables};
use crate::{
    instruction::{block::Block, local_variable::LocalVariableMap, pattern::{destruct_pattern::DestructPattern, Pattern}, set::Set, r#struct::Struct}, variable::Typed, Error
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
    instructions: Arc<[Instruction]>,
    lv_layer: LocalVariableMap,
) -> Result<Instruction, Error> {
    let sets = lv_layer
        .into_iter()
        .map(|(ident, var)| {
            let pattern = Pattern{
                destruct_pattern: DestructPattern::Ident(ident.clone()),
                var_type: var.as_type(),
            };
            Set{
                pattern,
                instruction: Instruction::LocalVariable(ident, var),
            }
        })
        .collect();
    let struct_ins = Struct {
        sets
    }
    .into();
    let instructions = [instructions, [struct_ins].into()].concat().into();
    Ok(Block { instructions }.into())
}
