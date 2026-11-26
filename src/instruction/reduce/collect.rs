use crate::{
    self as simplesl,
    errors::{ErrorWithSpan, IncorrectUnaryOperatorOperand},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithSpan, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext,
    },
    interpreter::Interpreter,
    stdlib::operators::collect,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type},
};
use simplesl_macros::var_type;
use std::sync::LazyLock;

pub static ACCEPTED_TYPE: LazyLock<Type> = LazyLock::new(|| var_type!(() -> (bool, any)));

#[derive(Debug)]
pub struct Collect {
    iterator: Instruction,
}

pub(crate) fn create(
    lhs: InstructionWithSpan,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let op = UnaryOperator::Collect;
    let return_type = lhs.return_type();
    let InstructionWithSpan {
        instruction: iterator,
        span,
    } = lhs;
    if return_type.iter_element().is_none() {
        let ins = parsing_context.slice(span).into();
        let error = IncorrectUnaryOperatorOperand {
            ins,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        }
        .into();
        errors.push(ErrorWithSpan { error, span });
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
        self as simplesl, Code, Interpreter, errors::IncorrectUnaryOperatorOperand,
        instruction::reduce::collect::ACCEPTED_TYPE, test::parse_unwrap_first_error,
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
                x:=() -> #Some(float)|#None {
                    val:=*i;
                    if val>70.0 {
                        return #None
                    }
                    i+=15.5;
                    return #Some(val); 
                }
                x$]"
            ),
            var!([45.5, 61.0])
        );
    }

    #[test]
    fn collect_mixed_iterator() {
        assert_eq!(parse_and_exec(r#"["a", 15, 1.7]~$]"#), var!(["a", 15, 1.7]));
    }

    #[test]
    fn collect_reject_array() {
        assert_eq!(
            parse_unwrap_first_error("[45, 15]$]"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
    }

    #[test]
    fn collect_reject_string() {
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$]"#),
            IncorrectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
            .into()
        );
    }

    #[test]
    fn collect_reject_noniterator_function() {
        assert_eq!(
            parse_unwrap_first_error(
                "x := (a:int) -> #Some(int) {
                    return #Some(13);
                }
                x$]"
            ),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int)-> #Some(int))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(
                "x := () -> int {
                    return 5;
                }
                x$]"
            ),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(()->int)
            }
            .into()
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
