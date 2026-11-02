use crate::{
    self as simplesl, BinOperator, Code, Error, Interpreter,
    function::Function,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables,
    },
    variable::{ReturnType, Type, Typed, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref MAP: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(func: () -> (#Some(any)|#None), mapper: (any) -> any) -> () -> (#Some(any)|#None) {
            return () -> #Some(any)|#None {
                if #Some(value) := func() {
                    return #Some(mapper(value));
                }
                return #None;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

#[derive(Debug)]
pub struct Map {
    lhs: Instruction,
    rhs: Instruction,
}

impl Map {
    pub fn create_instruction(
        lhs: Instruction,
        rhs: Instruction,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            errors.push(Error::CannotDo2(lhs_type, BinOperator::Map, rhs_type));
        }
        Self { lhs, rhs }.into()
    }

    pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Some(element_type) = lhs.iter_element() else {
            return false;
        };
        let expected_function = var_type!((element_type)->any);
        rhs.matches(&expected_function)
    }

    fn calc(iter: Variable, function: Variable) -> ExecResult {
        let result_type = function.as_type().return_type().unwrap();
        let result = MAP.exec(&[iter, function])?.into_function().unwrap();
        let mut result = Arc::unwrap_or_clone(result);
        result.return_type = var_type!(#Some(result_type)|#None);
        Ok(result.into())
    }

    fn return_type(rhs: Type) -> Type {
        let Some(element_type) = rhs.return_type() else {
            return Type::Never;
        };
        var_type!(()->(#Some(element_type)|#None))
    }
}

impl BaseInstruction for Map {}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        Self::calc(iter, function)
    }
}

impl Recreate for Map {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        Self {
            lhs: self.lhs.recreate(local_variables),
            rhs: self.rhs.recreate(local_variables),
        }
        .into()
    }
}

impl ReturnType for Map {
    fn return_type(&self) -> Type {
        Self::return_type(self.rhs.return_type())
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::{self as simplesl, ExecError};
    use crate::{Code, Interpreter, variable::Variable};
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((int)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((int|float)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((any)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((any)->float)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None) | () -> (#Some(float)|#None)),
            &var_type!((int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None)),
            &var_type!((any)->float | (int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None) | () -> (#Some(float)|#None)),
            &var_type!((int | float)->int | (int | float)->float)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None) | [float]),
            &var_type!((int)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None) | () -> (bool, float)),
            &var_type!((float)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (#Some(bool)|#None)),
            &var_type!((int)->int | int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (#Some(int)|#None) | float),
            &var_type!(( any)->int | (any)->any)
        ));
    }

    #[test]
    fn maps_iterator() {
        assert_eq!(
            parse_and_exec(
                "double := (value: int) -> int { return value * 2 };
                [1, 2, 3]~ @ double $]"
            ),
            Ok(Variable::from([2.into(), 4.into(), 6.into()]))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, ExecError> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
    }
}
