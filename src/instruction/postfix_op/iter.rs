use crate::{
    self as simplesl, Code, Error, Interpreter,
    function::Function,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref ITER: Arc<Function> = Code::parse(
        &Interpreter::with_stdlib(),
        "(array: [int]) -> () -> (#Some(int) | #None) {
            i := mut -1;
            len := std.len(array)
            return () -> #Some(int) | #None {
                i+=1;
                if *i < len {
                    return #Some(array[*i])
                }
                return #None
            }
        } "
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

#[derive(Debug)]
pub struct Iter(Instruction);

impl Iter {
    pub fn create_instruction(lhs: InstructionWithStr, errors: &mut Vec<Error>) -> Instruction {
        let op = UnaryOperator::Iter;
        let lhs_type = lhs.return_type();
        if !lhs_type.matches(&var_type!([any])) {
            errors.push(Error::IncorectUnaryOperatorOperand {
                ins: lhs.str,
                op,
                expected: var_type!([any]),
                given: lhs_type,
            });
        }
        Self(lhs.instruction).into()
    }
}

impl BaseInstruction for Iter {}

impl Exec for Iter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.0.exec(interpreter)?;
        let element_type = var.as_type().element_type();
        let result = ITER.exec(&[var]).unwrap().into_function().unwrap();
        let mut result = Arc::unwrap_or_clone(result);
        result.return_type =
            element_type.map_or_else(Type::none, |e_type| var_type!(#Some(e_type) | #None));
        Ok(result.into())
    }
}

impl Recreate for Iter {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instruction = self.0.recreate(local_variables);
        Self(instruction).into()
    }
}

impl ReturnType for Iter {
    fn return_type(&self) -> Type {
        let instruction_return_type = self.0.return_type();
        let Some(element_type) = instruction_return_type.element_type() else {
            return Type::Never;
        };
        var_type!(()->(#Some(element_type)|#None))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter,
        unary_operator::UnaryOperator,
        variable::{Typed, Variable},
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Iter;

    #[test]
    fn iter() {
        assert_eq!(
            parse_unwrap_first_error("45~"),
            Error::IncorectUnaryOperatorOperand {
                ins: "45".into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!(int)
            }
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"~"#),
            Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!(string)
            }
        );
        assert_eq!(
            parse_unwrap_first_error(
                "x := ()->[int]|string {
                    return [45, 15]
                }
                x()~"
            ),
            Error::IncorectUnaryOperatorOperand {
                ins: "x ()".into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!([int] | string)
            }
        );
        assert_eq!(
            parse_and_exec(
                "x:=[45, 5]~;
                (x(), x(), x())"
            ),
            var!((#Some(45), #Some(5), #None))
        );
        assert_eq!(
            parse_and_exec(
                "x:=[45, [5.5], true]~;
                (x(), x(), x())"
            ),
            var!((#Some(45), #Some([5.5]), #Some(true)))
        );
        let result = parse_and_exec(r#"[45, 5.5, "abc"]~"#);
        assert_eq!(
            result.as_type(),
            var_type!(()->(#Some(int|float|string)|#None))
        )
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
