use crate::{
    instruction::Instruction,
    variable::{ReturnType, Variable},
    Error,
};
pub mod assign_add;
pub mod assign_subtract;
use super::{BinOperation, BinOperator};

pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    let Some(var_type) = lhs_type.mut_element_type() else {
        return Err(Error::CannotDo2(lhs_type, "=", rhs_type));
    };
    if !rhs_type.matches(&var_type) {
        return Err(Error::CannotDo2(lhs_type, "=", rhs_type));
    }
    Ok(BinOperation {
        lhs,
        rhs,
        op: BinOperator::Assign,
    }
    .into())
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    let lhs = lhs.into_mut().unwrap();
    *lhs.variable.write().unwrap() = rhs.clone();
    rhs
}
