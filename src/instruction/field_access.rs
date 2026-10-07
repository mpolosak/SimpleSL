use super::{Instruction, InstructionWithStr};
use crate::{
    Error,
    instruction::{block::Block, set::Set, unary_operation::function_call},
    stdlib::operators::GetField,
    variable::{ReturnType, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(tuple: InstructionWithStr, op: Pair<Rule>) -> Result<Instruction, Error> {
    let return_type = tuple.return_type();
    if !return_type.is_struct() {
        return Err(Error::CannotFieldAccess(tuple.str, return_type));
    }
    let pair = op.into_inner().next().unwrap();
    let field_ident = Arc::from(pair.as_str());
    let Some(return_type) = return_type.field_type(&field_ident) else {
        return Err(Error::NoField {
            struct_ident: tuple.str,
            field_ident,
            struct_type: return_type,
        });
    };

    let variable_set = Set::new_ident("variable".into(), tuple.instruction).into();
    let field_set = Set::new_ident("field".into(), Variable::from(field_ident).into()).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(GetField).into_function().unwrap());
    function.return_type = return_type;
    let call = function_call(function);

    Ok(Block {
        instructions: [variable_set, field_set, call].into(),
    }
    .into())
}
