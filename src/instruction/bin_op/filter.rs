use super::Filter;
use crate::{
    instruction::{
        traits::{CanBeUsed, ExecResult},
        Exec,
    },
    interpreter::Interpreter,
    variable::{Array, FunctionType, ReturnType, Type, Variable},
};

impl CanBeUsed for Filter {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.index_result() else {
            return false;
        };
        let expected_function = FunctionType {
            params: [element_type.clone()].into(),
            return_type: Type::Int,
        } | FunctionType {
            params: [Type::Int, element_type].into(),
            return_type: Type::Int,
        };
        rhs.matches(&expected_function)
    }
}

impl Exec for Filter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
            unreachable!("Tried to do {array} ? {function}")
        };
        let array_iter = array.iter().cloned();
        let elements = if function.params.len() == 1 {
            array_iter
                .filter_map(|element| match function.exec(&[element.clone()]) {
                    Ok(Variable::Int(0)) => None,
                    Ok(_) => Some(Ok(element)),
                    e @ Err(_) => Some(e),
                })
                .collect::<Result<_, _>>()
        } else {
            array_iter
                .enumerate()
                .filter_map(|(index, element)| {
                    match function.exec(&[index.into(), element.clone()]) {
                        Ok(Variable::Int(0)) => None,
                        Ok(_) => Some(Ok(element)),
                        e @ Err(_) => Some(e),
                    }
                })
                .collect::<Result<_, _>>()
        }?;
        let element_type = array.element_type().clone();
        Ok(Array {
            element_type,
            elements,
        }
        .into())
    }
}

impl ReturnType for Filter {
    fn return_type(&self) -> Type {
        self.lhs.return_type()
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::{bin_op::Filter, traits::CanBeUsed};
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(Filter::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int)
        ));
        assert!(Filter::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int)
        ));
        assert!(Filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int|float)->int)
        ));
        assert!(Filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, any)->int)
        ));
        assert!(Filter::can_be_used(
            &var_type!([int] | [float | string]),
            &var_type!((any, int|float|string)->int)
        ));
        assert!(!Filter::can_be_used(
            &var_type!(int),
            &var_type!((any, any)->int)
        ));
        assert!(!Filter::can_be_used(
            &var_type!([int] | float),
            &var_type!((any, any)->int)
        ));
        assert!(!Filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, int)->int)
        ));
        assert!(!Filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float, any)->int)
        ));
        assert!(!Filter::can_be_used(
            &var_type!([int] | [float]),
            &var_type!(string)
        ))
    }
}
