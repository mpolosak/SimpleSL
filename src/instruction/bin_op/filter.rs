use super::Filter;
use crate::{
    instruction::{
        traits::{CanBeUsed, ExecResult},
        Exec, ExecStop,
    },
    interpreter::Interpreter,
    variable::{FunctionType, ReturnType, Type, Variable},
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
        if function.params.len() == 1 {
            return array_iter
                .filter_map(|element| match function.exec(&[element.clone()]) {
                    Ok(Variable::Int(0)) => None,
                    Ok(_) => Some(Ok(element)),
                    e @ Err(_) => Some(e),
                })
                .collect::<Result<_, _>>()
                .map_err(ExecStop::from);
        }
        array_iter
            .enumerate()
            .filter_map(
                |(index, element)| match function.exec(&[index.into(), element.clone()]) {
                    Ok(Variable::Int(0)) => None,
                    Ok(_) => Some(Ok(element)),
                    e @ Err(_) => Some(e),
                },
            )
            .collect::<Result<_, _>>()
            .map_err(ExecStop::from)
    }
}

impl ReturnType for Filter {
    fn return_type(&self) -> Type {
        self.lhs.return_type()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{bin_op::Filter, traits::CanBeUsed},
        variable::{parse_type, Type},
    };

    #[test]
    fn can_be_used() {
        assert!(Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("(int)->int")
        ));
        assert!(Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("(int, float)->int")
        ));
        assert!(!Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("(int)->float")
        ));
        assert!(Filter::can_be_used(
            &[Type::Int].into(),
            &parse_type!("(int)->int")
        ));
        assert!(Filter::can_be_used(
            &[Type::Int].into(),
            &parse_type!("(int, any)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("(int|float)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("(any, any)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float|string]"),
            &parse_type!("(any, int|float|string)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("int"),
            &parse_type!("(any, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|float"),
            &parse_type!("(any, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("(any, int)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("(float, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("string")
        ))
    }
}
