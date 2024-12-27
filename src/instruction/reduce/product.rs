use crate::{
    self as simplesl,
    instruction::{unary_operation::UnaryOperation, ExecResult, Instruction, InstructionWithStr},
    stdlib::operators::{FLOAT_PRODUCT, INT_PRODUCT},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
}

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Product;
    let return_type = array.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op,
    }
    .into())
}

pub fn exec(var: Variable) -> ExecResult {
    let return_type = var.as_type();
    if return_type.matches(&var_type!(() -> (bool, int))) {
        return Ok(INT_PRODUCT.as_function().unwrap().exec_with_args(&[var])?);
    }
    Ok(FLOAT_PRODUCT
        .as_function()
        .unwrap()
        .exec_with_args(&[var])?)
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::{
        instruction::reduce::product::ACCEPTED_TYPE, unary_operator::UnaryOperator,
        variable::Variable, Code, Error, Interpreter,
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
