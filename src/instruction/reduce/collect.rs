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
    fn collect() {
        assert_eq!(
            parse_and_exec("[45, 15]$]").unwrap_err()[0],
            Error::IncorectUnaryOperatorOperand {
                ins: "[45, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
        );
        assert_eq!(
            parse_and_exec(r#""abc"$]"#).unwrap_err()[0],
            Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
        );
        assert_eq!(
            parse_and_exec(
                "x := (a:int) -> (bool, int) {
                    return (true, 13);
                }
                x$]"
            )
            .unwrap_err()[0],
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int)->(bool, int))
            }
        );
        assert_eq!(
            parse_and_exec(
                "x := () -> int {
                    return 5;
                }
                x$]"
            )
            .unwrap_err()[0],
            Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(()->int)
            }
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
                    if val>70.0 {
                        return (false, val)
                    }
                    i+=15.5;
                    return (true, val); 
                }
                x$]"
            ),
            Ok(var!([45.5, 61.0]))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Vec<Error>> {
        Code::parse(&Interpreter::without_stdlib(), script).map(|code| code.exec().unwrap())
    }
}
