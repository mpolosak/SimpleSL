use crate::function::Function;
use crate::variable::Typed;
use crate::{self as simplesl, Code, Interpreter};
use crate::{
    instruction::ExecResult,
    variable::{Type, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref FILTER: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(func: () -> (bool, int), predicate: (int) -> bool) -> () -> (bool, int) {
            return () -> (bool, int) {
                loop {
                    res := func();
                    (con, value) := res;
                    if !con || predicate(value) return res;
                }
                return (false, 0);
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let Some(element_type) = lhs.iter_element() else {
        return false;
    };
    let expected_function = var_type!((element_type)->bool);
    rhs.matches(&expected_function)
}

pub fn exec(iter: Variable, function: Variable) -> ExecResult {
    let element = iter.as_type().return_type().unwrap();
    let result = FILTER
        .exec_with_args(&[iter, function])?
        .into_function()
        .unwrap();
    let mut result = Arc::unwrap_or_clone(result);
    result.return_type = element;
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
            &var_type!([int]),
            &var_type!((int)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int|float)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, any)->bool)
        ));
        assert!(filter::can_be_used(
            &var_type!([int] | [float | string]),
            &var_type!((any, int|float|string)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!(int),
            &var_type!((any, any)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | float),
            &var_type!((any, any)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, int)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float, any)->bool)
        ));
        assert!(!filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!(string)
        ))
    }
}
