use crate::{
    self as simplesl, instruction::{function::call::FunctionCall, tuple::Tuple, Instruction, InstructionWithStr}, stdlib::operators::{ALL, ANY}, unary_operator::UnaryOperator, variable::{ReturnType, Type, Variable}, Error
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, bool));
}

pub fn create(iterator: InstructionWithStr, op: UnaryOperator, errors: &mut Vec<Error>) -> Instruction {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        errors.push(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let function = if op == UnaryOperator::All {
        Variable::from(ALL)
    } else {
        Variable::from(ANY)
    }
    .into();

    let args = Tuple {
        elements: [iterator.instruction].into(),
    }
    .into();

    FunctionCall { function, args }.into()
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter,
        instruction::reduce::bool_reduce::ACCEPTED_TYPE, unary_operator::UnaryOperator,
        variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const ALL: UnaryOperator = UnaryOperator::All;
    const ANY: UnaryOperator = UnaryOperator::Any;

    #[test]
    fn all() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$&&"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$&&"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> bool {return true;} x$&&"),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> bool)
            }
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: bool) -> (bool, bool) {return (true, a);} x$&&"),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((bool) -> (bool, bool))
            }
        );
        assert_eq!(parse_and_exec("[true, false, true]~$&&"), Ok(var!(false)));
        assert_eq!(parse_and_exec("[true, true]~$&&"), Ok(var!(true)));
        assert_eq!(parse_and_exec("[]~$&&"), Ok(var!(true)));
        assert_eq!(
            parse_and_exec("[6.5, 6.5, 3.4]~$&&").unwrap_err()[0],
            Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            }
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$&&"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            }
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16, 3]~$&&"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16, 3] ~".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int))
            }
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->(bool, bool){
                    i+=1;
                    return (*i<20, *i<4);
                }
                (x$&&, *i)"
            ),
            Ok(var!((false, 4)))
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->(bool, bool){
                    i+=1;
                    return (*i<4, *i<4);
                }
                (x$&&, *i)"
            ),
            Ok(var!((true, 4)))
        )
    }

    #[test]
    fn any() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$||"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$||"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> bool {return true;} x$||"),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> bool)
            }
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: bool) -> (bool, bool) {return (true, a);} x$||"),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((bool) -> (bool, bool))
            }
        );
        assert_eq!(parse_and_exec("[true, false, true]~$||"), Ok(var!(true)));
        assert_eq!(parse_and_exec("[false, false]~$||"), Ok(var!(false)));
        assert_eq!(parse_and_exec("[]~$||"), Ok(var!(false)));
        assert_eq!(
            parse_unwrap_first_error("[6.5, 6.5, 3.4]~$||"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            }
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$||"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            }
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16, 3]~$||"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16, 3] ~".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int))
            }
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->(bool, bool){
                    i+=1;
                    return (*i<20, *i>4);
                }
                (x$||, *i)"
            ),
            Ok(var!((true, 5)))
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->(bool, bool){
                    i+=1;
                    return (*i<4, *i>4);
                }
                (x$||, *i)"
            ),
            Ok(var!((false, 4)))
        )
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Vec<Error>> {
        Code::parse(&Interpreter::without_stdlib(), script).map(|code| code.exec().unwrap())
    }
    fn parse_unwrap_first_error(script: &str) -> Error {
        Code::parse(&Interpreter::without_stdlib(), script).unwrap_err().swap_remove(0)
    }
}
