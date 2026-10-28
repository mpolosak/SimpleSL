use crate as simplesl;
use crate::variable::Type;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!((int, int) | (bool, bool));
}

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let lhs = lhs.clone();
    let rhs = rhs.clone();
    var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
}

#[cfg(test)]
mod tests {
    use crate::{
        BinOperator, Code, Error, Interpreter,
        instruction::bin_op::bitwise::can_be_used,
        variable::{Type, Variable},
    };

    #[test]
    fn can_be_used_int_int() {
        assert!(can_be_used(&Type::Int, &Type::Int))
    }

    #[test]
    fn can_be_used_bool_bool() {
        assert!(can_be_used(&Type::Bool, &Type::Bool))
    }

    #[test]
    fn can_be_used_bool_int() {
        assert!(!can_be_used(&Type::Bool, &Type::Int))
    }

    #[test]
    fn can_be_used_float_int() {
        assert!(!can_be_used(&Type::Float, &Type::Int))
    }

    #[test]
    fn can_be_used_bool_never() {
        assert!(can_be_used(&Type::Bool, &Type::Never))
    }

    #[test]
    fn can_be_used_never_int() {
        assert!(can_be_used(&Type::Never, &Type::Int))
    }

    #[test]
    fn can_be_used_never_never() {
        assert!(can_be_used(&Type::Never, &Type::Never))
    }

    #[test]
    fn can_be_used_string_never() {
        assert!(!can_be_used(&Type::String, &Type::Never))
    }

    #[test]
    fn can_be_used_never_void() {
        assert!(!can_be_used(&Type::Never, &Type::Void))
    }

    #[test]
    fn bitand_int_int() {
        assert_eq!(parse_and_exec("5&3"), Variable::Int(5 & 3))
    }

    #[test]
    fn bitand_bool_bool() {
        assert_eq!(parse_and_exec("true&false"), Variable::Bool(false))
    }

    #[test]
    fn bitand_bool_float() {
        assert_eq!(
            parse_unwrap_first_error("true&5.5"),
            Error::CannotDo2(Type::Bool, BinOperator::BitwiseAnd, Type::Float)
        )
    }

    #[test]
    fn bitor_int_int() {
        assert_eq!(parse_and_exec("5|3"), Variable::Int(5 | 3))
    }

    #[test]
    fn bitor_bool_bool() {
        assert_eq!(parse_and_exec("true|false"), Variable::Bool(true))
    }

    #[test]
    fn bitwise_float_int_int() {
        assert_eq!(
            parse_unwrap_errors("5.5|5&4"),
            vec![Error::CannotDo2(
                Type::Float,
                BinOperator::BitwiseOr,
                Type::Int
            )]
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
