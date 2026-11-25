use std::sync::LazyLock;

use crate::{
    self as simplesl, Error,
    errors::IncorrectUnaryOperatorOperand,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithSpan, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext,
    },
    interpreter::Interpreter,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
};
use simplesl_macros::var_type;

pub static ACCEPTED_TYPE: LazyLock<Type> = LazyLock::new(
    || var_type!(() -> (#Some(int)|#None) | () -> (#Some(float)|#None) | () -> (#Some(string)|#None)),
);

#[derive(Debug)]
pub struct Sum {
    iterator: Instruction,
}

impl Sum {
    pub fn create(
        iterator: InstructionWithSpan,
        parsing_context: &ParsingContext,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let return_type = iterator.return_type();
        if !return_type.matches(&ACCEPTED_TYPE) {
            let ins = parsing_context.slice(iterator.span).into();
            errors.push(
                IncorrectUnaryOperatorOperand {
                    ins,
                    op: UnaryOperator::Sum,
                    expected: ACCEPTED_TYPE.clone(),
                    given: return_type,
                }
                .into(),
            );
        }
        Self {
            iterator: iterator.instruction,
        }
        .into()
    }
}

impl BaseInstruction for Sum {}

impl Exec for Sum {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?;
        let return_type = iterator.as_type().iter_element().unwrap();
        let mut result = Variable::of_type(&return_type).unwrap();
        let iterator = iterator.into_function().unwrap();
        loop {
            let tagged = iterator.exec(&[])?.into_tagged().unwrap();
            if tagged.tag.as_ref() == "None" {
                break;
            }
            let value = tagged.payload.clone().unwrap();
            result = result + value;
        }
        Ok(result)
    }
}

impl Recreate for Sum {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iterator = self.iterator.recreate(local_variables);
        Self { iterator }.into()
    }
}

impl ReturnType for Sum {
    fn return_type(&self) -> Type {
        self.iterator
            .return_type()
            .iter_element()
            .unwrap_or(Type::Never)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, errors::IncorrectUnaryOperatorOperand,
        instruction::reduce::sum::ACCEPTED_TYPE, unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Sum;

    #[test]
    fn sum_rejects_arrays() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$+"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
    }

    #[test]
    fn sum_rejects_strings() {
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$+"#),
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
    fn sum_rejects_functions_with_invalid_return_types() {
        assert_eq!(
            parse_unwrap_first_error("x:= () -> int {return 5;} x$+"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: int) -> (bool, int) {return (true, a);} x$+"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            }
            .into()
        );
    }

    #[test]
    fn sum_adds_values_of_supported_types() {
        assert_eq!(parse_and_exec("[45, 16, 3]~$+"), var!(64));
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$+"), var!(19.4));
        assert_eq!(parse_and_exec(r#"["a", "6.5", "$"]~$+"#), var!("a6.5$"));
    }

    #[test]
    fn sum_rejects_mixed_numeric_iterator_types() {
        assert_eq!(
            parse_unwrap_first_error("[45, 16.5, 3]~$+"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 16.5, 3]~".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(int|float)|#None))
            }
            .into()
        );
    }

    #[test]
    fn sum_accepts_functions_returning_union_iterators() {
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(#Some(int)|#None)|() -> (#Some(float)|#None){
                    return [45, 16, 45]~;
                }
                x()$+"
            ),
            var!(106)
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(#Some(int)|#None)|() -> (#Some(float)|#None){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$+"
            ),
            var!(10.6)
        );
    }

    #[test]
    fn sum_empty_iterators_preserve_their_element_type() {
        assert_eq!(
            parse_and_exec("x:=() -> #Some(float)|#None { return #None } x$+"),
            var!(0.0)
        );
        assert_eq!(
            parse_and_exec(r#"x:=() -> #Some(string)|#None { return #None } x$+"#),
            var!("")
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }

    fn parse_unwrap_first_error(script: &str) -> Error {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap_err()
            .swap_remove(0)
    }
}
