use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::unary_operator::UnaryOperator;
use crate::variable::{ReturnType, Variable};
use crate::{self as simplesl, Interpreter};
use crate::{
    instruction::{Instruction, InstructionWithStr},
    variable::Type,
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, any));
}

pub(crate) fn create(lhs: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Collect;
    let return_type = lhs.return_type();
    if !can_be_used(&return_type) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: lhs.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: lhs.instruction,
        op,
    }
    .into())
}

pub fn can_be_used(lhs: &Type) -> bool {
    lhs.matches(&ACCEPTED_TYPE)
}

pub(crate) fn exec(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let mut vec = Vec::new();
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        vec.push(tuple[1].clone());
    }
    return Ok(vec.into());
}

pub(crate) fn return_type(lhs: Type) -> Type {
    let element = lhs.iter_element().unwrap();
    var_type!([element])
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::{
        instruction::reduce::collect::ACCEPTED_TYPE, unary_operator::UnaryOperator,
        variable::Variable, Code, Error, Interpreter,
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
