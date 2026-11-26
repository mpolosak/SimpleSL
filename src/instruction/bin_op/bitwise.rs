use crate as simplesl;
use crate::variable::Type;
use simplesl_macros::var_type;
use std::sync::LazyLock;

pub static ACCEPTED_TYPE: LazyLock<Type> = LazyLock::new(|| var_type!((int, int) | (bool, bool)));

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    let lhs = lhs.clone();
    let rhs = rhs.clone();
    var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
}

#[cfg(test)]
mod tests {
    use crate::{
        BinOperator, Code, Interpreter,
        errors::CannotDo2,
        instruction::bin_op::bitwise::can_be_used,
        test::{parse_unwrap_errors, parse_unwrap_first_error},
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
        assert!(!can_be_used(&Type::Never, &Type::none()))
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
            CannotDo2 {
                lhs: Type::Bool,
                op: BinOperator::BitwiseAnd,
                rhs: Type::Float
            }
            .into()
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
            vec![
                CannotDo2 {
                    lhs: Type::Float,
                    op: BinOperator::BitwiseOr,
                    rhs: Type::Int
                }
                .into()
            ]
        )
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
