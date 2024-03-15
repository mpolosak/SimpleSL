use crate::{
    instruction::{
        macros::binOp,
        traits::{CanBeUsed, ExecResult},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{FunctionType, ReturnType, Type, Variable},
};

binOp!(Filter, "?", cfi);

impl CanBeUsed for Filter {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.element_type() else {
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
        let mut new_array: Vec<Variable> = Vec::new();
        if function.params.len() == 1 {
            for element in array.iter().cloned() {
                if function.exec(&[element.clone()])? != Variable::Int(0) {
                    new_array.push(element);
                }
            }
            return Ok(new_array.into());
        }
        for (index, element) in array.iter().cloned().enumerate() {
            if function.exec(&[index.into(), element.clone()])? != Variable::Int(0) {
                new_array.push(element);
            }
        }
        Ok(new_array.into())
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
        instruction::{array_ops::Filter, traits::CanBeUsed},
        variable::{parse_type, Type},
    };

    #[test]
    fn can_be_used() {
        assert!(Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("function(int)->int")
        ));
        assert!(Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("function(int, float)->int")
        ));
        assert!(!Filter::can_be_used(
            &Type::EmptyArray,
            &parse_type!("function(int)->float")
        ));
        assert!(Filter::can_be_used(
            &[Type::Int].into(),
            &parse_type!("function(int)->int")
        ));
        assert!(Filter::can_be_used(
            &[Type::Int].into(),
            &parse_type!("function(int, any)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("function(int|float)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("function(any, any)->int")
        ));
        assert!(Filter::can_be_used(
            &parse_type!("[int]|[float|string]"),
            &parse_type!("function(any, int|float|string)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("int"),
            &parse_type!("function(any, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|float"),
            &parse_type!("function(any, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("function(any, int)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("function(float, any)->int")
        ));
        assert!(!Filter::can_be_used(
            &parse_type!("[int]|[float]"),
            &parse_type!("string")
        ))
    }
}
