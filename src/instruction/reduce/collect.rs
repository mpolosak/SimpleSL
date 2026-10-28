use crate::{
    self as simplesl, Error,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    stdlib::operators::collect,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, any));
}

#[derive(Debug)]
pub struct Collect {
    iterator: Instruction,
}

pub(crate) fn create(lhs: InstructionWithStr, errors: &mut Vec<Error>) -> Instruction {
    let op = UnaryOperator::Collect;
    let return_type = lhs.return_type();
    let InstructionWithStr {
        instruction: iterator,
        str: ins,
    } = lhs;
    if return_type.iter_element().is_none() {
        errors.push(Error::IncorectUnaryOperatorOperand {
            ins,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    Collect { iterator }.into()
}

impl BaseInstruction for Collect {}

impl Exec for Collect {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?.into_function().unwrap();
        Ok(collect(&iterator)?.into())
    }
}

impl Recreate for Collect {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iterator = self.iterator.recreate(local_variables);
        Self { iterator }.into()
    }
}

impl ReturnType for Collect {
    fn return_type(&self) -> Type {
        let element_type = self
            .iterator
            .return_type()
            .iter_element()
            .unwrap_or(Type::Never);
        var_type!([element_type])
    }
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
    fn collect_int_iterator() {
        assert_eq!(parse_and_exec("[45, 15, 17]~$]"), var!([45, 15, 17]));
    }

    #[test]
    fn collect_float_iterator() {
        assert_eq!(
            parse_and_exec(
                "i:=mut 45.5;
                x:=() -> (bool, float) {
                    val:=*i;
                    if val>70.0 {
                        return (false, val)
                    }
                    i+=15.5;
                    return (true, val); 
                }
                x$]"
            ),
            var!([45.5, 61.0])
        );
    }

    #[test]
    fn collect_mixed_iterator() {
        assert_eq!(
            parse_and_exec(r#"["a", 15, 1.7]~$]"#),
            var!(["a", 15, 1.7])
        );
    }

    #[test]
    fn collect_reject_array() {
        assert_eq!(
            parse_unwrap_first_error("[45, 15]$]"),
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
        );
    }

    #[test]
    fn collect_reject_string() {
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$]"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
        );
    }

    #[test]
    fn collect_reject_noniterator_function() {
        assert_eq!(
            parse_unwrap_first_error(
                "x := (a:int) -> (bool, int) {
                    return (true, 13);
                }
                x$]"
            ),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int)->(bool, int))
            }
        );
        assert_eq!(
            parse_unwrap_first_error(
                "x := () -> int {
                    return 5;
                }
                x$]"
            ),
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(()->int)
            }
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script).unwrap().exec().unwrap()
    }

    fn parse_unwrap_first_error(script: &str) -> Error {
        Code::parse(&Interpreter::without_stdlib(), script).unwrap_err().swap_remove(0)
    }
}
