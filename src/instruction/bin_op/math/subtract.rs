use crate::BinOperator;
use crate::instruction::{Instruction, create_from_instructions_with_exec};

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Subtract, |lhs, rhs| lhs - rhs)
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, BinOperator, Code, Error, Interpreter,
        variable::{Type, Variable},
    };
    use simplesl_macros::var;
    const OP: BinOperator = BinOperator::Subtract;

    #[test]
    fn subtract_int_from_int() {
        assert_eq!(parse_and_exec("4-1"), var!(3))
    }

    #[test]
    fn subtract_float_from_float() {
        assert_eq!(parse_and_exec("4.0-5.0"), var!(-1.0))
    }

    #[test]
    fn subtract_float_from_int() {
        assert_eq!(
            parse_unwrap_first_error("4-1.0"),
            Error::CannotDo2(Type::Int, OP, Type::Float)
        )
    }

    #[test]
    fn subtract_int_float_float() {
        assert_eq!(
            parse_unwrap_errors("4-1.0-5.5"),
            vec![Error::CannotDo2(Type::Int, OP, Type::Float)]
        )
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }

    fn parse_unwrap_first_error(script: &str) -> Error {
        parse_unwrap_errors(script).swap_remove(0)
    }

    fn parse_unwrap_errors(script: &str) -> Vec<Error> {
        Code::parse(&Interpreter::without_stdlib(), script).unwrap_err()
    }
}
