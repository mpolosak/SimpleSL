use super::macros::ACCEPTED_INT_TYPE as ACCEPTED_TYPE;
use super::{macros::binOpCBU, Instruction};
use crate::variable::{Type, Typed, Variable};
use duplicate::duplicate_item;

binOpCBU!(BitwiseAnd, "&");
binOpCBU!(BitwiseOr, "|");
binOpCBU!(Xor, "&");

#[duplicate_item(
    bitwise op1 op2;
    [BitwiseAnd] [lhs & rhs] [&];
    [BitwiseOr] [lhs | rhs] [|];
    [Xor] [lhs ^ rhs] [^];
)]
impl bitwise {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (op1).into(),
            (var @ Variable::Array(_), _) | (_, var @ Variable::Array(_))
                if var.as_type() == Type::EmptyArray =>
            {
                var
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
