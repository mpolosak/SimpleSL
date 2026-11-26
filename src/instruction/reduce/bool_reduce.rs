use crate::{
    self as simplesl,
    errors::{ErrorWithSpan, IncorrectUnaryOperatorOperand},
    instruction::{
        Instruction, InstructionWithSpan, function::call::FunctionCall,
        parsing_context::ParsingContext, tuple::Tuple,
    },
    stdlib::operators::{ALL, ANY},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
};
use simplesl_macros::var_type;
use std::sync::LazyLock;

pub static ACCEPTED_TYPE: LazyLock<Type> = LazyLock::new(|| var_type!(() -> (#Some(bool)|#None)));

pub fn create(
    iterator: InstructionWithSpan,
    op: UnaryOperator,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        let span = iterator.span;
        let ins = parsing_context.slice(iterator.span).into();
        let error = IncorrectUnaryOperatorOperand {
            ins,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        }
        .into();
        errors.push(ErrorWithSpan { error, span });
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
        errors::IncorrectUnaryOperatorOperand, instruction::reduce::bool_reduce::ACCEPTED_TYPE, test::{parse_and_exec, parse_unwrap_first_error}, unary_operator::UnaryOperator,
        self as simplesl
    };
    use simplesl_macros::{var, var_type};
    const ALL: UnaryOperator = UnaryOperator::All;
    const ANY: UnaryOperator = UnaryOperator::Any;

    #[test]
    fn all() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$&&"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$&&"#),
            IncorrectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> bool {return true;} x$&&"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> bool)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: bool) -> #Some(bool) {return #Some(a);} x$&&"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((bool) -> #Some(bool))
            }
            .into()
        );
        assert_eq!(parse_and_exec("[true, false, true]~$&&"), Ok(var!(false)));
        assert_eq!(parse_and_exec("[true, true]~$&&"), Ok(var!(true)));
        assert_eq!(parse_and_exec("[]~$&&"), Ok(var!(true)));
        assert_eq!(
            parse_unwrap_first_error("[6.5, 6.5, 3.4]~$&&"),
            IncorrectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4]~".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(float)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$&&"#),
            IncorrectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"]~"#.into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(string)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16, 3]~$&&"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 16, 3]~".into(),
                op: ALL,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(int)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->#Some(bool)|#None{
                    i+=1;
                    if *i<20 {
                        return #Some(*i<4);
                    } 
                    return #None;
                }
                (x$&&, *i)"
            ),
            Ok(var!((false, 4)))
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->#Some(bool)|#None{
                    i+=1;
                    if *i<4 {
                        return #Some(*i<4);
                    }
                    return #None;
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
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$||"#),
            IncorrectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> bool {return true;} x$||"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> bool)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: bool) -> #Some(bool) {return #Some(a);} x$||"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((bool) -> #Some(bool))
            }
            .into()
        );
        assert_eq!(parse_and_exec("[true, false, true]~$||"), Ok(var!(true)));
        assert_eq!(parse_and_exec("[false, false]~$||"), Ok(var!(false)));
        assert_eq!(parse_and_exec("[]~$||"), Ok(var!(false)));
        assert_eq!(
            parse_unwrap_first_error("[6.5, 6.5, 3.4]~$||"),
            IncorrectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4]~".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(float)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$||"#),
            IncorrectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"]~"#.into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#None | #Some(string)))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16, 3]~$||"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 16, 3]~".into(),
                op: ANY,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#None | #Some(int)))
            }
            .into()
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->#Some(bool) | #None {
                    i+=1;
                    if *i<20 {
                        return #Some(*i>4);
                    }
                    return #None;
                }
                (x$||, *i)"
            ),
            Ok(var!((true, 5)))
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 0;
                x:=()->#Some(bool)|#None{
                    i+=1;
                    if *i<4 {
                        return #Some(*i>4);
                    }
                    return #None;
                }
                (x$||, *i)"
            ),
            Ok(var!((false, 4)))
        )
    }
}
