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
    reduce::{All, Any, BitAndReduce, BitOrReduce, Product, Reduce, Sum},
    set::Set,
    type_filter::TypeFilter,
    FunctionCall, Instruction,
};
use crate::variable::ReturnType;
use duplicate::duplicate_item;
use std::{fmt::Debug, sync::Arc};
pub use {
    can_be_used::{can_be_used_int, can_be_used_num, CanBeUsed},
    exec::{Exec, ExecResult, ExecStop},
    recreate::Recreate,
};

pub trait BaseInstruction: Exec + Recreate + ReturnType + Debug + Sync + Send {}

#[duplicate_item(T; [Map]; [Reduce]; [TypeFilter]; [At];
    [BitwiseNot]; [And]; [Or]; [UnaryMinus]; [Not]; [Block]; [IfElse]; [Match];
    [SetIfElse]; [DestructTuple]; [FunctionCall]; [FunctionDeclaration]; [Import];
    [Return]; [Set]; [All]; [Any]; [Product];
    [BitAndReduce]; [BitOrReduce]; [Sum];
)]
impl BaseInstruction for T {}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self::Other(Arc::new(value))
    }
}
