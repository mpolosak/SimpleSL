use crate::instruction::bin_op::concat::Concat;
use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{BaseInstruction, Exec, ExecResult, Instruction, Recreate};
use crate::variable::{ReturnType, Type, Variable};
use crate::{BinOperator, Error, Interpreter};

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
        if lhs_rt.is_array() && rhs_rt.is_array() {
            return Concat { lhs, rhs }.into();
        }
        if !Self::can_be_used(&lhs_rt, &rhs_rt) {
            errors.push(Error::CannotDo2(lhs_rt, BinOperator::Add, rhs_rt));
        }
        Self { lhs, rhs }.into()
    }

    pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        matches!(lhs, Type::Int | Type::Float | Type::String) && lhs == rhs
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
    use crate::{self as simplesl, BinOperator, Code, Error, Interpreter, variable::Variable};
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

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }

    fn parse_unwrap_first_error(script: &str) -> Error {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap_err()
            .swap_remove(0)
    }
}
