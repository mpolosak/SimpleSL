use crate::{
    self as simplesl, BinOperator, Error,
    instruction::{Instruction, InstructionWithStr, bin_op::BinOperation, tuple::Tuple},
    stdlib::operators::{AND, OR},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int));
}

pub fn create(iterator: InstructionWithStr, op: UnaryOperator) -> Result<Instruction, Error> {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let lhs = if op == UnaryOperator::BitAnd {
        AND.clone().into()
    } else {
        OR.clone().into()
    };
    let rhs = Tuple {
        elements: [iterator].into(),
    }
    .into();
    Ok(BinOperation {
        lhs,
        rhs,
        op: BinOperator::FunctionCall,
    }
    .into())
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::bit::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const AND: UnaryOperator = UnaryOperator::BitAnd;
    const OR: UnaryOperator = UnaryOperator::BitOr;

    #[test]
    fn bitand_reduce() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$&"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$&"), Ok(var!(0)));
        assert_eq!(parse_and_exec("[5, 6, 7]~$&"), Ok(var!(4)));
        assert_eq!(parse_and_exec("[]~$&"), Ok(var!(-1)));
        assert_eq!(
            parse_and_exec("[6.5, 6.5, 3.4]~$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            })
        );
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$&"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
    }

    #[test]
    fn bitor_reduce() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$|"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$|"), Ok(var!(63)));
        assert_eq!(parse_and_exec("[5, 6, 7]~$|"), Ok(var!(7)));
        assert_eq!(parse_and_exec("[]~$|"), Ok(var!(0)));
        assert_eq!(
            parse_and_exec("[6.5, 6.5, 3.4]~$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            })
        );
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$|"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
