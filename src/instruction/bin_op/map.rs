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

#[derive(Debug)]
pub struct Map {
    lhs: Instruction,
    rhs: Instruction,
}

impl Map {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, BinOperator::Map, rhs_type));
        }
        Ok(Self { lhs, rhs }.into())
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
        let result = MAP
            .exec(&[iter, function])?
            .into_function()
            .unwrap();
        let mut result = Arc::unwrap_or_clone(result);
        result.return_type = var_type!((bool, result_type));
        Ok(result.into())
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

impl Map {
    fn return_type(rhs: Type) -> Type {
        let element_type = rhs.return_type().unwrap();
        var_type!(()->(bool, element_type))
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate as simplesl;
    use crate::{Code, Error, Interpreter, variable::Variable};
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int|float)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->float)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((any)->float | (int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((int | float)->int | (int | float)->float)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (bool, int) | [float]),
            &var_type!((int)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (bool, int) | () -> (bool, float)),
            &var_type!((float)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((float)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (bool, int)),
            &var_type!((int)->int | int)
        ));
        assert!(!Map::can_be_used(
            &var_type!(() -> (bool, int) | float),
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

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
