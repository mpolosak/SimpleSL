use crate::instruction::bin_op::concat::Concat;
use crate::instruction::local_variable::LocalVariables;
use crate::instruction::panic::Panic;
use crate::instruction::{BaseInstruction, Exec, ExecResult, Instruction, Recreate};
use crate::variable::{ReturnType, Type, Variable};
use crate::{self as simplesl, BinOperator, Error, Interpreter};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!((int, int) | (float, float) | (string, string));
}

#[derive(Debug)]
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl Add {
    pub fn create_instruction(
        lhs: Instruction,
        rhs: Instruction,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if lhs_rt.is_array() && rhs_rt.is_array() && (lhs_rt != Type::Never || rhs_rt != Type::Never) {
            return Concat { lhs, rhs }.into();
        }
        if !Self::can_be_used(lhs_rt.clone(), rhs_rt.clone()) {
            errors.push(Error::CannotDo2(lhs_rt, BinOperator::Add, rhs_rt));
            return Panic.into()
        }
        Self { lhs, rhs }.into()
    }

    pub fn can_be_used(lhs: Type, rhs: Type) -> bool {
        var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
    }
}

impl BaseInstruction for Add {}

impl Exec for Add {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(lhs + rhs)
    }
}

impl Recreate for Add {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let rhs = self.rhs.recreate(local_variables);
        let (Some(lhs), Some(rhs)) = (
            lhs.downcast_ref::<Variable>(),
            rhs.downcast_ref::<Variable>(),
        ) else {
            return Self { lhs, rhs }.into();
        };
        (lhs.clone() + rhs.clone()).into()
    }
}

impl ReturnType for Add {
    fn return_type(&self) -> Type {
        self.lhs.return_type()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{self as simplesl, instruction::bin_op::add::Add, variable::{Type, Variable}, BinOperator, Code, Error, Interpreter};
    use simplesl_macros::{var, var_type};

    #[test]
    fn test_add_integers() {
        assert_eq!(parse_and_exec("4+4"), Variable::Int(8));
    }

    #[test]
    fn test_add_floats() {
        assert_eq!(parse_and_exec("4.5+0.5"), Variable::Float(5.0));
    }

    #[test]
    fn test_add_strings() {
        assert_eq!(parse_and_exec(r#""aa" + "B""#), var!("aaB"));
    }

    #[test]
    fn test_add_arrays() {
        assert_eq!(parse_and_exec("[5, 5, 6] + [5]"), var!([5, 5, 6, 5]));
    }

    #[test]
    fn test_add_array_to_scalar_fails() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4, 5, 6] + 5"#),
            Error::CannotDo2(var_type!([int]), BinOperator::Add, var_type!(int))
        );
        assert_eq!(
            parse_unwrap_first_error(r#"[4.5, 5.7, 6.0] + 3.3"#),
            Error::CannotDo2(var_type!([float]), BinOperator::Add, var_type!(float))
        );
    }

    #[test]
    fn test_add_string_and_array_fails() {
        assert_eq!(
            parse_unwrap_first_error(r#""7" + ["a", "aaa"]"#),
            Error::CannotDo2(var_type!(string), BinOperator::Add, var_type!([string]))
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "aaa"]+"3""#),
            Error::CannotDo2(var_type!([string]), BinOperator::Add, var_type!(string))
        );
    }

    #[test]
    fn test_add_different_numeric_types_fails() {
        assert_eq!(
            parse_unwrap_first_error("4+4.5"),
            Error::CannotDo2(var_type!(int), BinOperator::Add, var_type!(float))
        );
    }

    #[test]
    fn test_add_strings_and_numbers_fails() {
        assert_eq!(
            parse_unwrap_first_error(r#""4"+4.5"#),
            Error::CannotDo2(var_type!(string), BinOperator::Add, var_type!(float))
        );
        assert_eq!(
            parse_unwrap_first_error(r#""4"+4"#),
            Error::CannotDo2(var_type!(string), BinOperator::Add, var_type!(int))
        );
    }

    #[test]
    fn test_add_array_and_float_fails() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4]+4.5"#),
            Error::CannotDo2(var_type!([int]), BinOperator::Add, var_type!(float))
        );
        assert_eq!(
            parse_unwrap_first_error(r#"[4, 5.5]+4.5"#),
            Error::CannotDo2(var_type!([int | float]), BinOperator::Add, var_type!(float))
        )
    }

    #[test]
    fn test_add_int_float_float() {
        assert_eq!(
            parse_unwrap_errors("4+5.5+5.5"),
            Vec::from([Error::CannotDo2(
                var_type!(int),
                BinOperator::Add,
                var_type!(float)
            )])
        )
    }

    #[test]
    fn test_add_int_float_float_string() {
        assert_eq!(
            parse_unwrap_errors(r#"(4+5.5)+(5.5+"")"#),
            Vec::from([
                Error::CannotDo2(var_type!(int), BinOperator::Add, var_type!(float)),
                Error::CannotDo2(var_type!(float), BinOperator::Add, var_type!(string))
            ])
        )
    }

    #[test]
    fn test_multiple_add() {
        assert_eq!(
            parse_unwrap_errors("[1] + (2 + 3.0) + [4]"),
            vec![
                Error::CannotDo2(var_type!(int), BinOperator::Add, var_type!(float))
            ]
        );
        assert_eq!(
            parse_unwrap_errors("1 + (2 + 3.0) + 4"),
            vec![
                Error::CannotDo2(var_type!(int), BinOperator::Add, var_type!(float))
            ]
        );
        assert_eq!(
            parse_unwrap_errors("(2 + 3.0) + [4] + 5"),
            vec![
                Error::CannotDo2(var_type!(int), BinOperator::Add, var_type!(float)),
                Error::CannotDo2(var_type!([int]), BinOperator::Add, var_type!(int))
            ]
        )
    }
    
    #[test]
    fn can_be_used_int_int() {
        assert!(Add::can_be_used(Type::Int, Type::Int))
    }

    #[test]
    fn can_be_used_float_float() {
        assert!(Add::can_be_used(Type::Float, Type::Float))
    }

    #[test]
    fn can_be_used_string_string() {
        assert!(Add::can_be_used(Type::String, Type::String))
    }

    #[test]
    fn can_be_used_int_never() {
        assert!(Add::can_be_used(Type::Int, Type::Never))
    }

    #[test]
    fn can_be_used_never() {
        assert!(Add::can_be_used(Type::Never, Type::Never))
    }

     #[test]
    fn can_be_used_int_float() {
        assert!(!Add::can_be_used(Type::Int, Type::Float))
    }

    #[test]
    fn can_be_used_float_string() {
        assert!(!Add::can_be_used(Type::Float, Type::String))
    }

    #[test]
    fn can_be_used_string_int() {
        assert!(!Add::can_be_used(Type::String, Type::Int))
    }

    #[test]
    fn can_be_used_bool_never() {
        assert!(!Add::can_be_used(Type::Bool, Type::Never))
    }

    #[test]
    fn can_be_used_never_void() {
        assert!(!Add::can_be_used(Type::Never, Type::Void))
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
