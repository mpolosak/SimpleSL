use crate::variable::Variable;
use crate::ExecError;

pub fn exec(base: Variable, exp: Variable) -> Result<Variable, ExecError> {
    match (base, exp) {
        (_, Variable::Int(exp)) if exp < 0 => Err(ExecError::NegativeExponent),
        (Variable::Int(base), Variable::Int(exp)) => Ok((base.wrapping_pow(exp as u32)).into()),
        (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
        (base, exp) => panic!("Tried to calc {base} * {exp}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl, BinOperator};
    use crate::{variable::Variable, Code, Error, Interpreter};
    use simplesl_macros::var_type;

    #[test]
    fn test_pow_operator() {
        assert_eq!(parse_and_exec("4 ** 4"), Ok(Variable::Int(256)));
        assert_eq!(parse_and_exec("4.0 ** 2.0"), Ok(Variable::Float(16.0)));
        assert_eq!(parse_and_exec("4.0 ** 0.5"), Ok(Variable::Float(2.0)));
        assert_eq!(parse_and_exec("50 ** 0"), Ok(Variable::Int(1)));
        assert_eq!(parse_and_exec("50.0 ** 0.0"), Ok(Variable::Float(1.0)));
        assert_eq!(parse_and_exec("2 ** -2"), Err(Error::NegativeExponent));
        assert_eq!(parse_and_exec("2.0 ** -2.0"), Ok(Variable::Float(0.25)));
        assert_eq!(
            parse_and_exec(r#"[4, 5, 6] ** 5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Pow,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] ** 3.3"#),
            Err(Error::CannotDo2(
                var_type!([float]),
                BinOperator::Pow,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""7" ** ["a", "aaa"]"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Pow,
                var_type!([string])
            ))
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]**"3""#),
            Err(Error::CannotDo2(
                var_type!([string]),
                BinOperator::Pow,
                var_type!(string)
            ))
        );
        assert_eq!(
            parse_and_exec("4**4.5"),
            Err(Error::CannotDo2(
                var_type!(int),
                BinOperator::Pow,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"**4.5"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Pow,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"**4"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Pow,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4]**4.5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Pow,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]**4.5"#),
            Err(Error::CannotDo2(
                var_type!([int | float]),
                BinOperator::Pow,
                var_type!(float)
            ))
        );
        parse_and_exec("256**256").unwrap();
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
