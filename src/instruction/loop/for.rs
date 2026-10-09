use crate::{
    self as simplesl, BinOperator, Error,
    instruction::{
        BinOperation, Instruction, Loop,
        block::Block,
        control_flow::If,
        ident::Ident,
        local_variable::{LocalVariable, LocalVariables},
        pattern::{Pattern, destruct_pattern::DestructPattern},
        set::Set,
    },
    variable::{ReturnType, Type, Variable},
};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

lazy_static! {
    static ref ITER: Arc<str> = "$iter".into();
}

lazy_static! {
    static ref CON: Arc<str> = "$con".into();
}

pub fn create_instruction(
    pair: Pair<Rule>,
    local_variables: &mut LocalVariables,
) -> Result<Instruction, Error> {
    let mut inner = pair.into_inner();
    let ident: Arc<str> = inner.next().unwrap().as_str().into();
    let iter = Instruction::new(inner.next().unwrap(), local_variables)?;
    let Some(iter_element) = iter.return_type().iter_element() else {
        return Err(Error::WrongType(
            "iterator".into(),
            var_type!(() -> (bool, any)),
        ));
    };
    let mut local_variables = local_variables.create_layer();
    local_variables.in_loop = true;
    local_variables.insert(ident.clone(), LocalVariable::Other(iter_element));
    let iter = Set::new_ident(ITER.clone(), iter).into();
    let iter_call: Instruction = BinOperation {
        lhs: Ident {
            ident: ITER.clone(),
            variable: LocalVariable::Other(var_type!(()->(bool, any))),
        }
        .into(),
        rhs: Variable::Tuple([].into()).into(),
        op: BinOperator::FunctionCall,
    }
    .into();
    let destruct: Instruction = Set {
        pattern: Pattern {
            destruct_pattern: DestructPattern::Tuple([CON.clone(), ident].into()),
            var_type: iter_call.return_type(),
        },
        instruction: iter_call,
    }
    .into();
    let instruction = Instruction::new(inner.next().unwrap(), &mut local_variables)?;
    let condition = Ident {
        ident: CON.clone(),
        variable: LocalVariable::Other(Type::Bool),
    }
    .into();
    let if_else = If {
        conditions: [condition].into(),
        if_true: instruction,
        else_instruction: Instruction::Break,
    }
    .into();
    let body = Block {
        instructions: [destruct, if_else].into(),
    }
    .into();
    let l: Instruction = Loop(body).into();
    Ok(Block {
        instructions: [iter, l].into(),
    }
    .into())
}
