use crate::{
    variable::{Type, Variable},
    ExecError,
};

pub fn can_be_used<T, S>(lhs: Type, rhs: Type, can_be_used: T, return_type: S) -> bool
where
    T: FnOnce(&Type, &Type) -> bool,
    S: FnOnce(&Type, &Type) -> Type,
{
    let Some(var_type) = lhs.mut_element_type() else {
        return false;
    };
    let can_be_used = can_be_used(&var_type, &rhs);
    let return_type = return_type(&var_type, &rhs);
    can_be_used && return_type.matches(&var_type)
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
