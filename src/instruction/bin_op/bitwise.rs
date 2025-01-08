use crate as simplesl;
use crate::variable::Type;
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!((int, int) | (bool, bool));
}

pub fn can_be_used(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
}

#[duplicate_item(
    Bitwise bitwise op1 op2;
    [BitwiseAnd] [bitwise_and] [lhs & rhs] [&];
    [BitwiseOr] [bitwise_or] [lhs | rhs] [|];
    [Xor] [xor] [lhs ^ rhs] [^];
)]
pub mod bitwise {
    use crate::{
        instruction::{BinOperation, Instruction},
        variable::Variable,
        BinOperator,
    };

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (lhs, rhs) => BinOperation {
                lhs,
                rhs,
                op: BinOperator::Bitwise,
            }
            .into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (op1).into(),
            (Variable::Bool(lhs), Variable::Bool(rhs)) => (op1).into(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
