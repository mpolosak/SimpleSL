use crate::variable::Typed;
use crate::{self as simplesl, Code, Interpreter};
use crate::{
    function::Function,
    instruction::ExecResult,
    variable::{Type, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref MAP: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(func: () -> (bool, int), mapper: (int) -> int) -> () -> (bool, int) {
            return () -> (bool, int) {
                res := func();
                (con, value) := res;
                if !con return res;
                return (true, mapper(value));
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
    let expected_function = var_type!((element_type)->any);
    rhs.matches(&expected_function)
}

pub fn exec(iter: Variable, function: Variable) -> ExecResult {
    let result_type = function.as_type().return_type().unwrap();
    let result = MAP
        .exec_with_args(&[iter, function])?
        .into_function()
        .unwrap();
    let mut result = Arc::unwrap_or_clone(result);
    result.return_type = var_type!((bool, result_type));
    Ok(result.into())
}

pub fn return_type(rhs: Type) -> Type {
    let element_type = rhs.return_type().unwrap();
    var_type!(()->(bool, element_type))
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::bin_op::map;
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        // true
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int|float)->any)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->int)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->float)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int | float)->int)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->float | (int | float)->int)
        ));
        assert!(map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int | float)->int | (int | float)->float)
        ));
        // false
        assert!(!map::can_be_used(
            &var_type!(() -> (bool, int) | [float]),
            &var_type!((int)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((float)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int | int)
        ));
        assert!(!map::can_be_used(
            &var_type!(() -> (bool, int) | float),
            &var_type!(( any)->int | (any)->any)
        ));
    }
}
