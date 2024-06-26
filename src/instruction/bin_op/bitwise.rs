use super::{BitwiseAnd, BitwiseOr, Xor};
use crate::variable::{Array, Variable};
use duplicate::duplicate_item;

#[duplicate_item(
    bitwise op1 op2;
    [BitwiseAnd] [lhs & rhs] [&];
    [BitwiseOr] [lhs | rhs] [|];
    [Xor] [lhs ^ rhs] [^];
)]
impl bitwise {
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (op1).into(),
            (value, Variable::Array(array)) | (Variable::Array(array), value) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|element| Self::exec(element, value.clone()))
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
