use super::{Instruction, InstructionWithStr};
use crate::{
    instruction::{function::call::FunctionCall, tuple::Tuple}, stdlib::operators::At, variable::{from_str::parse_int, ReturnType, Type, Variable}, Error
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(tuple: InstructionWithStr, op: Pair<Rule>, errors: &mut Vec<Error>) -> Instruction {
    let tuple_type = tuple.return_type();
    let Some(len) = tuple_type.min_tuple_len() else {
        errors.push(Error::CannotTupleAccess(tuple.str, tuple_type));
        return Variable::Void.into()
    };

    let pair = op.into_inner().next().unwrap();
    let index = match parse_int(pair) {
        Ok(index) => index as usize,
        Err(error) => {
            errors.push(error);
            0
        },
    };

    let return_type= tuple_type.tuple_element_at(index).unwrap_or_else(|| {
        errors.push(Error::TupleIndexTooBig(index, tuple.str, len));
        Type::Never
    });

    let mut function = Arc::unwrap_or_clone(Variable::from(At).into_function().unwrap());
    function.return_type = return_type;

    let function = Variable::from(function).into();
    let args = Tuple {
        elements: [tuple.instruction, Variable::from(index).into()].into(),
    }
    .into();

    FunctionCall { function, args }.into()
}
