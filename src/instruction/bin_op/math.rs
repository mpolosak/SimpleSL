pub mod add;
pub mod divide;
pub mod modulo;
pub mod multiply;
pub mod pow;
pub mod subtract;
use duplicate::duplicate_item;

#[duplicate_item(
    ord Ord oper;
    [greater] [Greater] [>]; [greater_equal] [GreaterOrEqual] [>=]; [lower] [Lower] [<]; [lower_equal] [LowerOrEqual] [<=];
)]
pub mod ord {
    use crate::{
        instruction::{BinOperation, Instruction},
        variable::{Array, Variable},
        BinOperator,
    };
    use match_any::match_any;
    use std::sync::Arc;

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
                op: BinOperator::Ord,
            }
            .into(),
        }
    }
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs oper rhs).into(),
            (lhs, Variable::Array(array)) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|rhs| exec(lhs.clone(), rhs))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            },
            (Variable::Array(array), rhs) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| exec(lhs, rhs.clone()))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            },
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", stringify!(op))
        }
    }
}
