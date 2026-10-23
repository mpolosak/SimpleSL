use std::sync::Arc;
use crate::{
    self as simplesl,
    Error,
    instruction::{
        Instruction, InstructionWithStr,
        control_flow::{Match, MatchArm, MatchPattern},
        function::call::FunctionCall,
        ident::Ident,
        pattern::Pattern,
        tuple::Tuple,
    },
    stdlib::operators::{FLOAT_PRODUCT, INT_PRODUCT},
    unary_operator::UnaryOperator,
    variable::{MultiType, ReturnType, Type, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
}

pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op: UnaryOperator::Product,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let iterator = iterator.instruction;
    let iter_element = return_type.iter_element().unwrap();
    if let Type::Multi(types) = iter_element {
        return Ok(create_match(iterator, types));
    }

    let function = match return_type.iter_element().unwrap() {
        Type::Int => Variable::from(INT_PRODUCT),
        Type::Float => Variable::from(FLOAT_PRODUCT),
        _ => unreachable!(),
    }
    .into();
    let args = Tuple {
        elements: [iterator].into(),
    }
    .into();

    Ok(FunctionCall { function, args }.into())
}

fn create_match(iterator: Instruction, types: MultiType) -> Instruction {
    let iter: Arc<str> = "iter".into();
    let arms = types
        .iter()
        .map(|t| match t {
            Type::Int => MatchArm {
                pattern: MatchPattern::Pattern(Pattern::new_ident_pattern(
                    iter.clone(),
                    var_type!(() -> (bool, int)),
                )),
                guard: None,
                instruction: {
                    let function = Variable::from(INT_PRODUCT).into();
                    let args = Tuple {
                        elements: [Ident {
                            ident: iter.clone(),
                            variable: var_type!(() -> (bool, int)).into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into();

                    FunctionCall { function, args }.into()
                },
            },
            Type::Float => MatchArm {
                pattern: MatchPattern::Pattern(Pattern::new_ident_pattern(
                    "iter".into(),
                    var_type!(() -> (bool, float)),
                )),
                guard: None,
                instruction: {
                    let function = Variable::from(FLOAT_PRODUCT).into();
                    let args = Tuple {
                        elements: [Ident {
                            ident: iter.clone(),
                            variable: var_type!(() -> (bool, float)).into(),
                        }
                        .into()]
                        .into(),
                    }
                    .into();

                    FunctionCall { function, args }.into()
                },
            },
            _ => unreachable!(),
        })
        .collect();
    Match {
        expression: iterator,
        arms,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::product::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Product;

    #[test]
    fn product() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$*"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$*"), Ok(var!(2160)));
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$*"), Ok(var!(264.55)));
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$*"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [45, 16, 45]~;
                }
                x()$*"
            ),
            Ok(var!(32400))
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$*"
            ),
            Ok(var!(32.4))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
