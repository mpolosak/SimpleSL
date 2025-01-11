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
        instruction::{create_from_instructions_with_exec, Instruction},
        variable::Variable,
        BinOperator,
    };
    use match_any::match_any;

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        create_from_instructions_with_exec(lhs, rhs, BinOperator::Ord, exec)
    }
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs oper rhs).into(),
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", stringify!(op))
        }
    }
}
