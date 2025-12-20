use crate::{
    self as simplesl, instruction::{
        block::Block, control_flow::IfElse, local_variable::{LocalVariable, LocalVariables}, pattern::{destruct_pattern::DestructPattern, Pattern}, set::Set, BinOperation, Instruction, InstructionWithStr, Loop
    }, variable::{ReturnType, Type, Variable}, BinOperator, Error
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
    let iter = InstructionWithStr::new(inner.next().unwrap(), local_variables)?;
    let Some(iter_element) = iter.return_type().iter_element() else {
        return Err(Error::WrongType(
            "iterator".into(),
            var_type!(() -> (bool, any)),
        ));
    };
    let mut local_variables = local_variables.create_layer();
    local_variables.in_loop = true;
    local_variables.insert(ident.clone(), LocalVariable::Other(iter_element));
    let str = format!("$iter = {}", iter.str).into();
    let iter = InstructionWithStr {
        instruction: Set {
            pattern: Pattern::new_ident_pattern(ITER.clone(), iter.return_type()),
            instruction: iter,
        }
        .into(),
        str,
    };
    let iter_call = InstructionWithStr {
        instruction: BinOperation {
            lhs: Instruction::LocalVariable(
                ITER.clone(),
                LocalVariable::Other(var_type!(()->(bool, any))),
            ),
            rhs: Variable::Tuple([].into()).into(),
            op: BinOperator::FunctionCall,
        }
        .into(),
        str: "$iter()".into(),
    };
    let str = format!("($con, {ident}) = {}", iter_call.str).into();
    let destruct = InstructionWithStr {
        instruction: Set {
            pattern: Pattern { destruct_pattern: DestructPattern::Tuple([CON.clone(), ident].into()), var_type: iter_call.return_type() },
            instruction: iter_call,
        }
        .into(),
        str,
    };
    let instruction = InstructionWithStr::new(inner.next().unwrap(), &mut local_variables)?;
    let condition = InstructionWithStr {
        instruction: Instruction::LocalVariable(CON.clone(), LocalVariable::Other(Type::Bool)),
        str: CON.clone(),
    };
    let if_false = InstructionWithStr {
        instruction: Instruction::Break,
        str: "break".into(),
    };
    let str = format!("if $con {} else break", instruction.str).into();
    let if_else = InstructionWithStr {
        instruction: IfElse {
            condition,
            if_true: instruction,
            if_false,
        }
        .into(),
        str,
    };
    let str = format!("{{{}\n{}}}", destruct.str, if_else.str).into();
    let body = Block {
        instructions: [destruct, if_else].into(),
    }
    .into();
    let body = InstructionWithStr {
        instruction: body,
        str,
    };
    let str = format!("loop {}", body.str).into();
    let l = InstructionWithStr {
        instruction: Loop(body).into(),
        str,
    };
    Ok(Block {
        instructions: [iter, l].into(),
    }
    .into())
}
