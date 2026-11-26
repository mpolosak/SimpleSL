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
        self as simplesl, BinOperator, ExecError,
        errors::CannotDo2,
        test::{parse_and_exec, parse_unwrap_first_error},
        variable::Variable,
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
            parse_unwrap_first_error("4**4.5"),
            CannotDo2 {
                lhs: var_type!(int),
                op: BinOperator::Pow,
                rhs: var_type!(float)
            }
            .into()
        );
    }

    #[test]
    fn int_array_to_int() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4, 5, 6] ** 5"#),
            CannotDo2 {
                lhs: var_type!([int]),
                op: BinOperator::Pow,
                rhs: var_type!(int)
            }
            .into()
        );
    }

    #[test]
    fn string_to_string_array() {
        assert_eq!(
            parse_unwrap_first_error(r#""7" ** ["a", "aaa"]"#),
            CannotDo2 {
                lhs: var_type!(string),
                op: BinOperator::Pow,
                rhs: var_type!([string])
            }
            .into()
        );
    }

    #[test]
    fn string_array_to_string() {
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "aaa"]**"3""#),
            CannotDo2 {
                lhs: var_type!([string]),
                op: BinOperator::Pow,
                rhs: var_type!(string)
            }
            .into()
        );
    }

    #[test]
    fn float_array_to_float() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4.5, 5.7, 6.0] ** 3.3"#),
            CannotDo2 {
                lhs: var_type!([float]),
                op: BinOperator::Pow,
                rhs: var_type!(float)
            }
            .into()
        );
    }

    #[test]
    fn string_to_float() {
        assert_eq!(
            parse_unwrap_first_error(r#""4"**4.5"#),
            CannotDo2 {
                lhs: var_type!(string),
                op: BinOperator::Pow,
                rhs: var_type!(float)
            }
            .into()
        );
    }

    #[test]
    fn string_to_int() {
        assert_eq!(
            parse_unwrap_first_error(r#""4"**4"#),
            CannotDo2 {
                lhs: var_type!(string),
                op: BinOperator::Pow,
                rhs: var_type!(int)
            }
            .into()
        );
    }

    #[test]
    fn int_to_float_array() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4]**4.5"#),
            CannotDo2 {
                lhs: var_type!([int]),
                op: BinOperator::Pow,
                rhs: var_type!(float)
            }
            .into()
        );
    }

    #[test]
    fn int_float_array_to_float() {
        assert_eq!(
            parse_unwrap_first_error(r#"[4, 5.5]**4.5"#),
            CannotDo2 {
                lhs: var_type!([int | float]),
                op: BinOperator::Pow,
                rhs: var_type!(float)
            }
            .into()
        );
    }
}
