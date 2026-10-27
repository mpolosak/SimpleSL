use crate::ExecError;
use crate::variable::Variable;

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
    use crate::{
        self as simplesl, BinOperator, Code, Error, ExecError, Interpreter, variable::Variable,
    };
    use simplesl_macros::var_type;

    #[test]
    fn test_pow_operator() {
        assert_eq!(parse_and_exec("4 ** 4"), Ok(Variable::Int(256)));
        assert_eq!(parse_and_exec("4.0 ** 2.0"), Ok(Variable::Float(16.0)));
        assert_eq!(parse_and_exec("4.0 ** 0.5"), Ok(Variable::Float(2.0)));
        assert_eq!(parse_and_exec("50 ** 0"), Ok(Variable::Int(1)));
        assert_eq!(parse_and_exec("50.0 ** 0.0"), Ok(Variable::Float(1.0)));
        assert_eq!(parse_and_exec("2.0 ** -2.0"), Ok(Variable::Float(0.25)));
        parse_and_exec("256**256").unwrap();
    }

    #[test]
    fn negative_exponent() {
        assert_eq!(parse_and_exec("2 ** -2"), Err(ExecError::NegativeExponent));
    }

    #[test]
    fn int_to_float() {
        assert_eq!(
            parse("4**4.5").unwrap_err(),
            Error::CannotDo2(var_type!(int), BinOperator::Pow, var_type!(float))
        );
    }

    #[test]
    fn int_array_to_int() {
        assert_eq!(
            parse(r#"[4, 5, 6] ** 5"#).unwrap_err(),
            Error::CannotDo2(var_type!([int]), BinOperator::Pow, var_type!(int))
        );
    }

    #[test]
    fn string_to_string_array() {
        assert_eq!(
            parse(r#""7" ** ["a", "aaa"]"#).unwrap_err(),
            Error::CannotDo2(var_type!(string), BinOperator::Pow, var_type!([string]))
        );
    }

    #[test]
    fn string_array_to_string() {
        assert_eq!(
            parse(r#"["a", "aaa"]**"3""#).unwrap_err(),
            Error::CannotDo2(var_type!([string]), BinOperator::Pow, var_type!(string))
        );
    }

    #[test]
    fn float_array_to_float() {
        assert_eq!(
            parse(r#"[4.5, 5.7, 6.0] ** 3.3"#).unwrap_err(),
            Error::CannotDo2(var_type!([float]), BinOperator::Pow, var_type!(float))
        );
    }

    #[test]
    fn string_to_float() {
        assert_eq!(
            parse(r#""4"**4.5"#).unwrap_err(),
            Error::CannotDo2(var_type!(string), BinOperator::Pow, var_type!(float))
        );
    }

    #[test]
    fn string_to_int() {
        assert_eq!(
            parse(r#""4"**4"#).unwrap_err(),
            Error::CannotDo2(var_type!(string), BinOperator::Pow, var_type!(int))
        );
    }

    #[test]
    fn int_to_float_array() {
        assert_eq!(
            parse(r#"[4]**4.5"#).unwrap_err(),
            Error::CannotDo2(var_type!([int]), BinOperator::Pow, var_type!(float))
        );
    }

    #[test]
    fn int_float_array_to_float() {
        assert_eq!(
            parse(r#"[4, 5.5]**4.5"#).unwrap_err(),
            Error::CannotDo2(var_type!([int | float]), BinOperator::Pow, var_type!(float))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, ExecError> {
        parse(script).unwrap().exec()
    }

    fn parse(script: &str) -> Result<Code, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
    }
}
