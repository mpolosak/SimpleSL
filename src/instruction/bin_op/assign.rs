use super::{BinOperation, BinOperator};
use crate::{
    instruction::Instruction,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};

pub fn create_op<T, S>(
    lhs: Instruction,
    rhs: Instruction,
    op: BinOperator,
    can_be_used: T,
    return_type: S,
) -> Result<Instruction, Error>
where
    T: FnOnce(&Type, &Type) -> bool,
    S: FnOnce(&Type, &Type) -> Type,
{
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    let Some(var_type) = lhs_type.mut_element_type() else {
        return Err(Error::CannotDo2(lhs_type, op, rhs_type));
    };
    let can_be_used = can_be_used(&var_type, &rhs_type);
    let return_type = return_type(&var_type, &rhs_type);
    if !can_be_used || !return_type.matches(&var_type) {
        return Err(Error::CannotDo2(lhs_type, op, rhs_type));
    }
    Ok(BinOperation { lhs, rhs, op }.into())
}

pub fn exec<T: FnOnce(Variable, Variable) -> Variable>(
    lhs: Variable,
    rhs: Variable,
    function: T,
) -> Variable {
    let lhs = lhs.into_mut().unwrap();
    let mut lhs = lhs.variable.write().unwrap();
    *lhs = function(lhs.clone(), rhs);
    lhs.clone()
}

pub fn try_exec<T: FnOnce(Variable, Variable) -> Result<Variable, ExecError>>(
    lhs: Variable,
    rhs: Variable,
    function: T,
) -> Result<Variable, ExecError> {
    let lhs = lhs.into_mut().unwrap();
    let mut lhs = lhs.variable.write().unwrap();
    *lhs = function(lhs.clone(), rhs)?;
    Ok(lhs.clone())
}
