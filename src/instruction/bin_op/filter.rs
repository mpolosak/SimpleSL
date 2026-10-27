use crate::{
    self as simplesl, BinOperator, Error, Interpreter,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
    },
    stdlib::operators::FILTER,
    variable::{ReturnType, Type, Typed, Variable},
};
use simplesl_macros::var_type;
use std::sync::Arc;

#[derive(Debug)]
pub struct Filter {
    lhs: Instruction,
    rhs: Instruction,
}

impl Filter {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, BinOperator::Filter, rhs_type));
        }
        Ok(Self { lhs, rhs }.into())
    }

    pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.iter_element() else {
            return false;
        };
        let expected_function = var_type!((element_type)->bool);
        rhs.matches(&expected_function)
    }

    fn calc(iter: Variable, function: Variable) -> ExecResult {
        let return_type = iter.as_type().return_type().unwrap();
        let result = Variable::from(FILTER)
            .into_function()
            .unwrap()
            .exec(&[iter, function])?
            .into_function()
            .unwrap();
        let mut result = Arc::unwrap_or_clone(result);
        result.return_type = return_type;
        Ok(result.into())
    }
}

impl BaseInstruction for Filter {}

impl Exec for Filter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        Self::calc(iter, function)
    }
}

impl Recreate for Filter {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        Self {
            lhs: self.lhs.recreate(local_variables),
            rhs: self.rhs.recreate(local_variables),
        }
        .into()
    }
}

impl ReturnType for Filter {
    fn return_type(&self) -> Type {
        self.lhs.return_type()
    }
}

#[cfg(test)]
mod tests {
    use super::Filter;
    use crate::{self as simplesl, ExecError};
    use crate::{Code, Interpreter, variable::Variable};
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(Filter::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->bool)
        ));
        assert!(Filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int|float)->bool)
        ));
        assert!(Filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((any)->bool)
        ));
        assert!(Filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float|string)),
            &var_type!((int|float|string)->bool)
        ));
        assert!(!Filter::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any, int)->bool)
        ));
        assert!(!Filter::can_be_used(
            &var_type!([int] | () -> (bool, float)),
            &var_type!((any)->bool)
        ));
        assert!(!Filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int)->bool)
        ));
        assert!(!Filter::can_be_used(
            &var_type!(() -> (bool, int)| () -> (bool, float)),
            &var_type!((float, any)->bool)
        ));
        assert!(!Filter::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!(string)
        ));
    }

    #[test]
    fn filters_iterator() {
        assert_eq!(
            parse_and_exec(
                "is_even := (value: int) -> bool { return value % 2 == 0 };
                [1, 2, 3, 4]~ ? is_even $]"
            ),
            Ok(Variable::from([2.into(), 4.into()]))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, ExecError> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
    }
}
