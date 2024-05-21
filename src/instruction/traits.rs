mod can_be_used;
mod exec;
mod recreate;
use super::{
    at::At,
    bin_op::*,
    block::Block,
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    function::FunctionDeclaration,
    import::Import,
    prefix_op::{BitwiseNot, Not, UnaryMinus},
    r#return::Return,
    reduce::{
        All, Any, BitAndReduce, BitOrReduce, FloatProduct, FloatSum, IntProduct, IntSum, Reduce,
        StringSum,
    },
    set::Set,
    type_filter::TypeFilter,
    FunctionCall, Instruction,
};
use crate::variable::ReturnType;
use duplicate::duplicate_item;
use std::{fmt::Debug, sync::Arc};
pub use {
    can_be_used::CanBeUsed,
    exec::{Exec, ExecResult, ExecStop},
    recreate::Recreate,
};

pub trait BaseInstruction: Exec + Recreate + ReturnType + Debug + Sync + Send {}

#[duplicate_item(T; [Filter]; [Map]; [Reduce]; [TypeFilter]; [At];
    [BitwiseAnd]; [BitwiseOr]; [BitwiseNot]; [Xor]; [And]; [Or]; [Add]; [Subtract]; [Pow];
    [Multiply]; [Divide]; [Modulo]; [Equal]; [Greater]; [GreaterOrEqual]; [Lower];
    [LowerOrEqual]; [LShift]; [RShift]; [UnaryMinus]; [Not]; [Block]; [IfElse]; [Match];
    [SetIfElse]; [DestructTuple]; [FunctionCall]; [FunctionDeclaration]; [Import];
    [Return]; [Set]; [FloatSum]; [IntSum]; [StringSum]; [IntProduct]; [FloatProduct]; [All]; [Any];
    [BitAndReduce]; [BitOrReduce]
)]
impl BaseInstruction for T {}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self::Other(Arc::new(value))
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
