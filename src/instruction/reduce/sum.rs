use crate::{
    self as simplesl, instruction::{
        block::Block, control_flow::{Match, MatchArm, MatchPattern}, pattern::Pattern, set::Set, unary_operation::UnaryOperation, Instruction, InstructionWithStr
    }, stdlib::operators::{FLOAT_SUM, INT_SUM, STRING_SUM}, unary_operator::UnaryOperator, variable::{MultiType, ReturnType, Type, Variable}, Error
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!(() -> (bool, int) | () -> (bool, float) | () -> (bool, string));
}

pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op: UnaryOperator::Sum,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let iterator = iterator.instruction;
    let iter_element = return_type.iter_element().unwrap();
    if let Type::Multi(types) = iter_element {
        return  Ok(create_match(iterator, types))
    }
    
    let call = match return_type.iter_element().unwrap() {
        Type::Int => {
            function_call(INT_SUM)
        }
        Type::Float => {
            function_call(FLOAT_SUM)
        }
        Type::String => {
            function_call(STRING_SUM)
        }
        _ => unreachable!(),
    };
    let set = Set {
        pattern: Pattern::new_ident_pattern("iter".into(), return_type),
        instruction: iterator,
    }
    .into();
    Ok(Block{
        instructions: [set, call].into(),
    }.into())
}

fn create_match(iterator: Instruction, types: MultiType) -> Instruction {
    let arms = types.iter().map(|t| match t {
        Type::Int => MatchArm {
            pattern: MatchPattern::Pattern(Pattern::new_ident_pattern(
                "iter".into(),
                var_type!(() -> (bool, int)),
            )),
            instruction: function_call(INT_SUM),
        },
        Type::Float => MatchArm {
            pattern: MatchPattern::Pattern(Pattern::new_ident_pattern(
                "iter".into(),
                var_type!(() -> (bool, float)),
            )),
            instruction: function_call(FLOAT_SUM),
        },
        Type::String => MatchArm {
            pattern: MatchPattern::Pattern(Pattern::new_ident_pattern(
                "iter".into(),
                var_type!(() -> (bool, string)),
            )),
            instruction: function_call(STRING_SUM),
        },
        _ => unreachable!()
    }).collect();
    Match{ expression: iterator, arms }.into()
}

fn function_call<T: Into<Variable>>(function: T) -> Instruction {
    UnaryOperation {
        instruction: function.into().into(),
        op: UnaryOperator::FunctionCall,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::sum::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Sum;

    #[test]
    fn sum() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$+"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$+"), Ok(var!(64)));
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$+"), Ok(var!(19.4)));
        assert_eq!(parse_and_exec(r#"["a", "6.5", "$"]~$+"#), Ok(var!("a6.5$")));
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$+"),
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
                x()$+"
            ),
            Ok(var!(106))
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$+"
            ),
            Ok(var!(10.6))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
