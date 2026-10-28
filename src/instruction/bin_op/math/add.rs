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
    pub fn create_instruction(lhs: Instruction, rhs: Instruction, errors: &mut Vec<Error>) -> Instruction {
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
    fn test_add_operator() {
        assert_eq!(parse_and_exec("4+4"), Ok(Variable::Int(8)));
        assert_eq!(parse_and_exec("4.5+0.5"), Ok(Variable::Float(5.0)));
        assert_eq!(parse_and_exec(r#""aa" + "B""#), Ok(var!("aaB")));
        assert_eq!(parse_and_exec("[5, 5, 6] + [5]"), Ok(var!([5, 5, 6, 5])));
        assert_eq!(
            parse_and_exec(r#"[4, 5, 6] + 5"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(int)
            )
        );
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] + 3.3"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!([float]),
                BinOperator::Add,
                var_type!(float)
            )
        );
        assert_eq!(
            parse_and_exec(r#""7" + ["a", "aaa"]"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!([string])
            )
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]+"3""#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!([string]),
                BinOperator::Add,
                var_type!(string)
            )
        );
        assert_eq!(
            parse_and_exec("4+4.5").unwrap_err()[0],
            Error::CannotDo2(
                var_type!(int),
                BinOperator::Add,
                var_type!(float)
            )
        );
        assert_eq!(
            parse_and_exec(r#""4"+4.5"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(float)
            )
        );
        assert_eq!(
            parse_and_exec(r#""4"+4"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(int)
            )
        );
        assert_eq!(
            parse_and_exec(r#"[4]+4.5"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(float)
            )
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#).unwrap_err()[0],
            Error::CannotDo2(
                var_type!([int | float]),
                BinOperator::Add,
                var_type!(float)
            )
        )
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Vec<Error>> {
        Code::parse(&Interpreter::without_stdlib(), script).map(|code| code.exec().unwrap())
    }
}
