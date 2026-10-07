use std::sync::Arc;

use crate::{
    self as simplesl, Error,
    instruction::{
        Instruction, InstructionWithStr, block::Block, set::Set, unary_operation::function_call,
    },
    stdlib::operators::Collect,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, any));
}

pub(crate) fn create(lhs: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Collect;
    let return_type = lhs.return_type();
    let Some(element_type) = return_type.iter_element() else {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: lhs.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    };

    let iter_set = Set::new_ident("iter".into(), lhs.instruction).into();

    let mut function = Arc::unwrap_or_clone(Variable::from(Collect).into_function().unwrap());
    function.return_type = var_type!([element_type]);
    let call = function_call(function);

    Ok(Block {
        instructions: [iter_set, call].into(),
    }
    .into())
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::collect::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Collect;

    #[test]
    fn collect() {
        assert_eq!(
            parse_and_exec("[45, 15]$]"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$]"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec(
                "x := (a:int) -> (bool, int) {
                    return (true, 13);
                }
                x$]"
            ),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int)->(bool, int))
            })
        );
        assert_eq!(
            parse_and_exec(
                "x := () -> int {
                    return 5;
                }
                x$]"
            ),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(()->int)
            })
        );
        assert_eq!(parse_and_exec("[45, 15, 17]~$]"), Ok(var!([45, 15, 17])));
        assert_eq!(
            parse_and_exec(r#"["a", 15, 1.7]~$]"#),
            Ok(var!(["a", 15, 1.7]))
        );
        assert_eq!(
            parse_and_exec(
                "i:=mut 45.5;
                x:=() -> (bool, float) {
                    val:=*i;
                    if val>70.0 return (false, val)
                    i+=15.5;
                    return (true, val); 
                }
                x$]"
            ),
            Ok(var!([45.5, 61.0]))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
