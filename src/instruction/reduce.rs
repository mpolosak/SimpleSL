pub mod bit;
pub mod bool_reduce;
pub mod collect;
pub mod product;
pub mod sum;
use crate::{
    self as simplesl, instruction::{
        function::call::FunctionCall, local_variable::LocalVariables, tuple::Tuple, Instruction, InstructionWithStr
    }, stdlib::operators::REDUCE, variable::{ReturnType, Type, Variable}, Error
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;
use std::sync::Arc;

pub fn create_instruction(
    iter: InstructionWithStr,
    initial_value: Pair<Rule>,
    function: InstructionWithStr,
    local_variables: &LocalVariables,
    errors: &mut Vec<Error>
) -> Instruction {
    let initial_value = InstructionWithStr::new_expression(initial_value, local_variables, errors);
    let iter_type = iter.return_type();
    let element_type = iter_type.iter_element().unwrap_or_else(|| {
        errors.push(Error::CannotReduce(iter.str));
        Type::Never
    });
    let function_return_type = function.return_type();
    let return_type = function_return_type.return_type().unwrap_or_else(|| {
        let element_type = element_type.clone();
        errors.push(Error::WrongType(
            "function".into(),
            var_type!((any, element_type)->any),
        ));
        Type::Never
    });
    let acc_type = initial_value.return_type() | element_type.clone() | return_type.clone();
    let expected_function = var_type!((acc_type, element_type)->return_type);
    if !function.return_type().matches(&expected_function) {
        errors.push(Error::WrongType("function".into(), expected_function));
    }

    let mut reduce = Arc::unwrap_or_clone(Variable::from(REDUCE).into_function().unwrap());
    reduce.return_type = function_return_type.return_type().unwrap_or(Type::Never);

    let args = Tuple {
        elements: [
            iter.instruction,
            initial_value.instruction,
            function.instruction,
        ]
        .into(),
    }
    .into();
    let function = Variable::from(reduce).into();

    FunctionCall { function, args }.into()
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl, Code, Error, Interpreter, variable::Variable};
    use simplesl_macros::var_type;

    #[test]
    fn reduce() {
        let interpreter = Interpreter::without_stdlib();
        assert_eq!(
            Code::parse(&interpreter, "[45, 67, 13] $0 5").unwrap_err()[0],
            Error::CannotReduce("[45, 67, 13]".into())
        );
        assert_eq!(
            Code::parse(&interpreter, r#""abc" $0 5"#).unwrap_err()[0],
            Error::CannotReduce(r#""abc""#.into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() {} $0 5").unwrap_err()[0],
            Error::CannotReduce("() {}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> int {return 0} $0 5").unwrap_err()[0],
            Error::CannotReduce("() -> int {return 0}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (any, any) {return (true, 0)} $0 5").unwrap_err()[0],
            Error::CannotReduce("() -> (any, any) {return (true, 0)}".into())
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (bool, any) {return (true, 0)} $0 5").unwrap_err()[0],
            Error::WrongType("function".into(), var_type!((any, any)->any))
        );
        assert_eq!(
            Code::parse(&interpreter, "() -> (bool, int) {return (true, 0)} $0 5").unwrap_err()[0],
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: any, b:float) -> any {}"
            )
            .unwrap_err()[0],
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: int, b: int) -> any {}"
            )
            .unwrap_err()[0],
            Error::WrongType("function".into(), var_type!((any, int)->any))
        );
        assert_eq!(
            Code::parse(
                &interpreter,
                "() -> (bool, int) {return (true, 0)} $0 (a: int, b:int) -> float { return 0.5 }"
            )
            .unwrap_err()[0],
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
