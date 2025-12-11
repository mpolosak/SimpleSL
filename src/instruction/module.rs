use std::sync::Arc;

use super::{Instruction, InstructionWithStr, local_variable::LocalVariables};
use crate::{
    Error,
    instruction::{block::Block, local_variable::LocalVariableMap, r#struct::Struct},
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
    instructions: Arc<[InstructionWithStr]>,
    lv_layer: LocalVariableMap,
) -> Result<Instruction, Error> {
    let (idents, values): (Vec<_>, Vec<_>) = lv_layer
        .iter()
        .map(|(ident, var)| {
            (
                ident.clone(),
                InstructionWithStr {
                    instruction: Instruction::LocalVariable(ident.clone(), var.clone()),
                    str: ident.clone(),
                },
            )
        })
        .unzip();
    let struct_ins = Struct {
        idents: idents.into(),
        values: values.into(),
    }
    .into();
    let struct_ins = InstructionWithStr {
        instruction: struct_ins,
        str: "struct".into(),
    };
    let instructions = [instructions, [struct_ins].into()].concat().into();
    Ok(Block { instructions }.into())
}
