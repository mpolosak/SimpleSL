pub mod bit;
pub mod bool_reduce;
pub mod collect;
pub mod product;
pub mod sum;
use crate::{
    self as simplesl, Error, ExecError,
    instruction::{
        Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Reduce {
    iter: InstructionWithStr,
    initial_value: InstructionWithStr,
    function: InstructionWithStr,
}

impl Reduce {
    pub fn create_instruction(
        iter: InstructionWithStr,
        initial_value: Pair<Rule>,
        function: InstructionWithStr,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let initial_value = InstructionWithStr::new_expression(initial_value, local_variables)?;
        let Some(element_type) = iter.return_type().iter_element() else {
            return Err(Error::CannotReduce(iter.str));
        };
        let Some(return_type) = function.return_type().return_type() else {
            return Err(Error::WrongType(
                "function".into(),
                var_type!((any, element_type)->any),
            ));
        };
        let acc_type = initial_value.return_type() | element_type.clone() | return_type.clone();
        let expected_function = var_type!((acc_type, element_type)->return_type);
        if !function.return_type().matches(&expected_function) {
            return Err(Error::WrongType("function".into(), expected_function));
        }
        Ok(Self {
            iter,
            initial_value,
            function,
        }
        .into())
    }
}

impl Recreate for Reduce {
    fn recreate(
        &self,
        local_variables: &mut crate::instruction::local_variable::LocalVariables,
    ) -> Result<Instruction, ExecError> {
        let array = self.iter.recreate(local_variables)?;
        let initial_value = self.initial_value.recreate(local_variables)?;
        let function = self.function.recreate(local_variables)?;
        Ok(Self {
            iter: array,
            initial_value,
            function,
        }
        .into())
    }
}

impl Exec for Reduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iter = self.iter.exec(interpreter)?;
        let initial_value = self.initial_value.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        let (Variable::Function(iter), Variable::Function(function)) = (&iter, &function) else {
            unreachable!("Tried to do {iter} ${initial_value} {function}")
        };
        let mut result = initial_value;
        while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
            if tuple[0] == Variable::Bool(false) {
                break;
            };
            result = function.exec_with_args(&[result, tuple[1].clone()])?;
        }
        Ok(result)
    }
}

impl ReturnType for Reduce {
    fn return_type(&self) -> Type {
        self.function.return_type().return_type().unwrap() | self.initial_value.return_type()
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl, Code, Error, Interpreter, variable::Variable};
    use simplesl_macros::var_type;

    #[test]
    fn reduce() {
        let interpreter = Interpreter::without_stdlib();
        assert_eq!(
            Code::parse(&interpreter, "[45, 67, 13] $0 5").unwrap_err(),
            Error::CannotReduce("[45, 67, 13]".into())
        );
        assert_eq!(
            Code::parse(&interpreter, r#""abc" $0 5"#).unwrap_err(),
            Error::CannotReduce(r#""abc""#.into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() {} $0 5").unwrap_err(),
            Error::CannotReduce("() {}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> int {return 0} $0 5").unwrap_err(),
            Error::CannotReduce("() -> int {return 0}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (any, any) {return (true, 0)} $0 5").unwrap_err(),
            Error::CannotReduce("() -> (any, any) {return (true, 0)}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (bool, any) {return (true, 0)} $0 5").unwrap_err(),
            Error::WrongType("function".into(), var_type!((any, any)->any))
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (bool, int) {return (true, 0)} $0 5").unwrap_err(),
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: any, b:float) -> any {}"
            )
            .unwrap_err(),
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: int, b: int) -> any {}"
            )
            .unwrap_err(),
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: int, b:int) -> float { return 0.5 }"
            )
            .unwrap_err(),
            Error::WrongType("function".into(), var_type!((int | float, int)->float))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (false, 0)} $0 (a: int, b:int) -> int { return 0 }"
            )
            .unwrap()
            .exec(),
            Ok(Variable::Int(0))
        );
    }
}
