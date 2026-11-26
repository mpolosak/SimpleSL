use crate::{
    self as simplesl,
    errors::{ErrorWithRange, IncorrectUnaryOperatorOperand},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithRange, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext,
    },
    interpreter::Interpreter,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
};
use simplesl_macros::var_type;
use std::sync::LazyLock;

pub static ACCEPTED_TYPE: LazyLock<Type> =
    LazyLock::new(|| var_type!(() -> (#Some(int)|#None) | () -> (#Some(float)|#None)));

#[derive(Debug)]
pub struct Product {
    iterator: Instruction,
}

impl Product {
    pub fn create(
        iterator: InstructionWithRange,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithRange>,
    ) -> Instruction {
        let return_type = iterator.return_type();
        if !return_type.matches(&ACCEPTED_TYPE) {
            let range = iterator.range;
            let ins = parsing_context.slice(range.clone()).into();
            let error = IncorrectUnaryOperatorOperand {
                ins,
                op: UnaryOperator::Product,
                expected: ACCEPTED_TYPE.clone(),
                given: return_type,
            }
            .into();
            errors.push(ErrorWithRange { error, range });
        }
        Self {
            iterator: iterator.instruction,
        }
        .into()
    }

    fn identity(element_type: &Type) -> Variable {
        if let Type::Int = element_type {
            return 1.into();
        }
        1.0.into()
    }
}

impl BaseInstruction for Product {}

impl Exec for Product {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?;
        let return_type = iterator.as_type().iter_element().unwrap();
        let mut result = Self::identity(&return_type);
        let iterator = iterator.into_function().unwrap();
        loop {
            let tagged = iterator.exec(&[])?.into_tagged().unwrap();
            if tagged.tag.as_ref() == "None" {
                break;
            }
            result = result * tagged.payload.clone().unwrap();
        }
        Ok(result)
    }
}

impl Recreate for Product {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iterator = self.iterator.recreate(local_variables);
        Self { iterator }.into()
    }
}

impl ReturnType for Product {
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
        self as simplesl, Code, Interpreter, errors::IncorrectUnaryOperatorOperand,
        instruction::reduce::product::ACCEPTED_TYPE, test::parse_unwrap_first_error,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Product;

    #[test]
    fn product_rejects_non_iterators() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$*"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$*"#),
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
    fn product_rejects_invalid_iterator_function_types() {
        assert_eq!(
            parse_unwrap_first_error("x:= () -> int {return 5;} x$*"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: int) -> #Some(int) {return #Some(a);} x$*"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> #Some(int))
            }
            .into()
        );
    }

    #[test]
    fn product_multiplies_integers() {
        assert_eq!(parse_and_exec("[45, 16, 3]~$*"), var!(2160));
    }

    #[test]
    fn product_multiplies_floats() {
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$*"), var!(264.55));
    }

    #[test]
    fn product_rejects_string_and_mixed_numeric_iterators() {
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$*"#),
            IncorrectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"]~"#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(string)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16.5, 3]~$*"),
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
    fn product_uses_integer_type_from_union_iterator() {
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(#Some(int)|#None)|() -> (#Some(float)|#None){
                    return [45, 16, 45]~;
                }
                x()$*"
            ),
            var!(32400)
        );
    }

    #[test]
    fn product_uses_float_type_from_union_iterator() {
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(#Some(int)|#None)|() -> (#Some(float)|#None){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$*"
            ),
            var!(32.4)
        );
    }

    #[test]
    fn product_empty_iterators_preserve_their_element_type() {
        assert_eq!(
            parse_and_exec("x:=() -> #Some(int)|#None { return #None } x$*"),
            var!(1)
        );
        assert_eq!(
            parse_and_exec("x:=() -> #Some(float)|#None { return #None} x$*"),
            var!(1.0)
        );
    }

    #[test]
    fn product_empty_union_iterator_uses_its_concrete_type() {
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(#Some(int)|#None)|() -> (#Some(float)|#None) {
                    return () -> #Some(float)|#None { return #None; };
                }
                x()$*"
            ),
            var!(1.0)
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
