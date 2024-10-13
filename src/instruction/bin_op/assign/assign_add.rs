use crate::{
    instruction::{add, BinOperation, BinOperator, Instruction},
    variable::{ReturnType, Variable},
    Error,
};

pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    let Some(var_type) = lhs_type.mut_element_type() else {
        return Err(Error::CannotDo2(lhs_type, "+=", rhs_type));
    };
    let can_be_used = add::can_be_used(&var_type, &rhs_type);
    let return_type = add::return_type(var_type.clone(), rhs_type.clone());
    if !can_be_used || !return_type.matches(&var_type) {
        return Err(Error::CannotDo2(lhs_type, "+=", rhs_type));
    }
    Ok(BinOperation {
        lhs,
        rhs,
        op: BinOperator::AssignAdd,
    }
    .into())
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    let lhs = lhs.into_mut().unwrap();
    let mut lhs = lhs.variable.write().unwrap();
    *lhs = add::exec(lhs.clone(), rhs);
    lhs.clone()
}
