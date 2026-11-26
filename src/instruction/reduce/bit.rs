use crate::{
    self as simplesl,
    errors::{ErrorWithSpan, IncorrectUnaryOperatorOperand},
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithSpan, Recreate,
        local_variable::LocalVariables, parsing_context::ParsingContext,
    },
    interpreter::Interpreter,
    stdlib::operators::{bitand_reduce, bitor_reduce},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
};
use simplesl_macros::var_type;
use std::sync::LazyLock;

pub static ACCEPTED_TYPE: LazyLock<Type> = LazyLock::new(|| var_type!(() -> (#Some(int)|#None)));

#[derive(Debug)]
pub struct AndReduce {
    iterator: Instruction,
}

#[derive(Debug)]
pub struct OrReduce {
    iterator: Instruction,
}

pub fn create(
    iterator: InstructionWithSpan,
    op: UnaryOperator,
    parsing_context: &ParsingContext,
    errors: &mut Vec<ErrorWithSpan>,
) -> Instruction {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        let span = iterator.span;
        let ins = parsing_context.slice(iterator.span).into();
        let error = IncorrectUnaryOperatorOperand {
            ins,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        }
        .into();
        errors.push(ErrorWithSpan { error, span });
    }
    let iterator = iterator.instruction;
    if op == UnaryOperator::BitAnd {
        AndReduce { iterator }.into()
    } else {
        OrReduce { iterator }.into()
    }
}

impl BaseInstruction for AndReduce {}

impl Exec for AndReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?.into_function().unwrap();
        Ok(Variable::from(bitand_reduce(iterator)?))
    }
}

impl Recreate for AndReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iterator = self.iterator.recreate(local_variables);
        Self { iterator }.into()
    }
}

impl ReturnType for AndReduce {
    fn return_type(&self) -> Type {
        Type::Int
    }
}

impl BaseInstruction for OrReduce {}

impl Exec for OrReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let iterator = self.iterator.exec(interpreter)?.into_function().unwrap();
        Ok(Variable::from(bitor_reduce(iterator)?))
    }
}

impl Recreate for OrReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let iterator = self.iterator.recreate(local_variables);
        Self { iterator }.into()
    }
}

impl ReturnType for OrReduce {
    fn return_type(&self) -> Type {
        Type::Int
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Interpreter, errors::IncorrectUnaryOperatorOperand,
        instruction::reduce::bit::ACCEPTED_TYPE, test::parse_unwrap_first_error,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const AND: UnaryOperator = UnaryOperator::BitAnd;
    const OR: UnaryOperator = UnaryOperator::BitOr;

    #[test]
    fn bitand_reduce() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$&"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$&"#),
            IncorrectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> int {return 5;} x$&"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: int) -> (bool, int) {return (true, a);} x$&"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            }
            .into()
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$&"), var!(0));
        assert_eq!(parse_and_exec("[5, 6, 7]~$&"), var!(4));
        assert_eq!(parse_and_exec("[]~$&"), var!(-1));
        assert_eq!(
            parse_unwrap_first_error("[6.5, 6.5, 3.4]~$&"),
            IncorrectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4]~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(float)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$&"#),
            IncorrectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"]~"#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(string)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16.5, 3]~$&"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 16.5, 3]~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(int|float)|#None))
            }
            .into()
        );
    }

    #[test]
    fn bitor_reduce() {
        assert_eq!(
            parse_unwrap_first_error("[45, 76, 15]$|"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#""abc"$|"#),
            IncorrectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= () -> int {return 5;} x$|"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("x:= (a: int) -> (bool, int) {return (true, a);} x$|"),
            IncorrectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            }
            .into()
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$|"), var!(63));
        assert_eq!(parse_and_exec("[5, 6, 7]~$|"), var!(7));
        assert_eq!(parse_and_exec("[]~$|"), var!(0));
        assert_eq!(
            parse_unwrap_first_error("[6.5, 6.5, 3.4]~$|"),
            IncorrectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4]~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(float)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error(r#"["a", "6.5", "$"]~$|"#),
            IncorrectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"]~"#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(string)|#None))
            }
            .into()
        );
        assert_eq!(
            parse_unwrap_first_error("[45, 16.5, 3]~$|"),
            IncorrectUnaryOperatorOperand {
                ins: "[45, 16.5, 3]~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (#Some(int|float)|#None))
            }
            .into()
        );
    }

    #[test]
    fn bitand_reduction_consumes_the_iterator_until_its_end() {
        assert_eq!(
            parse_and_exec(
                "count := mut 0
                iter := () -> #Some(int) | #None  {
                    count += 1
                    if *count < 4 {
                        return #Some(*count)
                    }
                    return #None
                }
                (iter$&, *count)"
            ),
            var!((0, 4))
        );
    }

    #[test]
    fn bitor_reduction_consumes_the_iterator_until_its_end() {
        assert_eq!(
            parse_and_exec(
                "count := mut 0
                iter := () -> #Some(int) | #None {
                    count += 1
                    if *count < 4 {
                        return #Some(-*count)
                    }
                    return #None
                }
                (iter$|, *count)"
            ),
            var!((-1, 4))
        );
    }

    fn parse_and_exec(script: &str) -> Variable {
        Code::parse(&Interpreter::without_stdlib(), script)
            .unwrap()
            .exec()
            .unwrap()
    }
}
