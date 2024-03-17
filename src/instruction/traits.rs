mod can_be_used;
mod exec;
mod recreate;
use super::{
    array::Array,
    array_ops::{Filter, Map, Reduce, TypeFilter},
    array_repeat::ArrayRepeat,
    at::At,
    bitwise::{BitwiseAnd, BitwiseOr, Xor},
    block::Block,
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    equal::Equal,
    function::FunctionDeclaration,
    import::Import,
    local_variable::LocalVariables,
    logic::{And, Or},
    math::{Add, Divide, Modulo, Multiply, Pow, Subtract},
    ord::{Greater, GreaterOrEqual, Lower, LowerOrEqual},
    prefix_op::{BitwiseNot, Not, UnaryMinus},
    r#return::Return,
    set::Set,
    shift::{LShift, RShift},
    FunctionCall, Instruction,
};
use crate::{interpreter::Interpreter, parse::Rule, variable::ReturnType, Error};
use duplicate::duplicate_item;
use pest::iterators::Pair;
use std::{fmt::Debug, rc::Rc};
pub use {
    can_be_used::CanBeUsed,
    exec::{Exec, ExecResult, ExecStop},
    recreate::Recreate,
};
pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error>;
}

pub trait MutCreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error>;
}

pub trait BaseInstruction: Exec + Recreate + ReturnType + Debug {}

#[duplicate_item(T; [Filter]; [Map]; [Reduce]; [TypeFilter]; [ArrayRepeat]; [Array]; [At];
    [BitwiseAnd]; [BitwiseOr]; [BitwiseNot]; [Xor]; [And]; [Or]; [Add]; [Subtract]; [Pow];
    [Multiply]; [Divide]; [Modulo]; [Equal]; [Greater]; [GreaterOrEqual]; [Lower];
    [LowerOrEqual]; [LShift]; [RShift]; [UnaryMinus]; [Not]; [Block]; [IfElse]; [Match];
    [SetIfElse]; [DestructTuple]; [FunctionCall]; [FunctionDeclaration]; [Import];
    [Return]; [Set]
)]
impl BaseInstruction for T {}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self::Other(Rc::new(value))
    }
}

pub trait ToResult<T, E> {
    fn to_result(self) -> Result<T, E>;
}

impl<T, E> ToResult<T, E> for T {
    fn to_result(self) -> Result<T, E> {
        Ok(self)
    }
}

impl<T, E0, E1: From<E0>> ToResult<T, E1> for Result<T, E0> {
    fn to_result(self) -> Result<T, E1> {
        self.map_err(E1::from)
    }
}
