use crate::{
    self as simplesl, instruction::ExecResult, stdlib::operators::FILTER, variable::{Type, Typed, Variable}
};
use simplesl_macros::var_type;
use std::sync::Arc;

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let Some(element_type) = lhs.iter_element() else {
        return false;
    };
    let expected_function = var_type!((element_type)->bool);
    rhs.matches(&expected_function)
}

pub fn exec(iter: Variable, function: Variable) -> ExecResult {
    let return_type = iter.as_type().return_type().unwrap();
    let result = Variable::from(FILTER)
        .into_function()
        .unwrap()
        .exec_with_args(&[iter, function])?
        .into_function()
        .unwrap();
    let mut result = Arc::unwrap_or_clone(result);
    result.return_type = return_type;
    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::bin_op::filter;
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(filter::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int|float)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((any)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float|string)),
            &var_type!((int|float|string)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any, int)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | () -> (bool, float)),
            &var_type!((any)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!(() -> (bool, int)| () -> (bool, float)),
            &var_type!((float, any)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!(string)
        ))
    }
}
