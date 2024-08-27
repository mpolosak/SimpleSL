use crate as simplesl;
use crate::variable::Type;
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!((int, int | [int]) | ([int], int) | (bool, bool | [bool]) | ([bool], bool));
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
    use std::sync::Arc;

    use crate::{
        instruction::{BinOperation, BinOperator, Instruction},
        variable::{Array, ReturnType, Variable},
        Error,
    };

    use super::can_be_used;

    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used(lhs_type.clone(), rhs_type.clone()) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::Bitwise,
        }
        .into())
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
            (Instruction::ArrayRepeat(array_repeat), rhs) => Arc::unwrap_or_clone(array_repeat)
                .map(|lhs| create_from_instructions(lhs, rhs))
                .into(),
            (lhs, Instruction::ArrayRepeat(array_repeat)) => Arc::unwrap_or_clone(array_repeat)
                .map(|rhs| create_from_instructions(lhs, rhs))
                .into(),
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
            (value, Variable::Array(array)) | (Variable::Array(array), value) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|element| exec(element, value.clone()))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            }
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
