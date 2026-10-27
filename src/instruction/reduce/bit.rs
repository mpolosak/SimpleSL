use crate::{
    self as simplesl, Error,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    stdlib::operators::{bitand_reduce, bitor_reduce},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int));
}

#[derive(Debug)]
pub struct AndReduce {
    iterator: Instruction,
}

#[derive(Debug)]
pub struct OrReduce {
    iterator: Instruction,
}

pub fn create(iterator: InstructionWithStr, op: UnaryOperator) -> Result<Instruction, Error> {
    let return_type = iterator.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: iterator.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    let iterator = iterator.instruction;
    if op == UnaryOperator::BitAnd {
        Ok(AndReduce { iterator }.into())
    } else {
        Ok(OrReduce { iterator }.into())
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
        self as simplesl, Code, Error, Interpreter, instruction::reduce::bit::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const AND: UnaryOperator = UnaryOperator::BitAnd;
    const OR: UnaryOperator = UnaryOperator::BitOr;

    #[test]
    fn bitand_reduce() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$&"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$&"), Ok(var!(0)));
        assert_eq!(parse_and_exec("[5, 6, 7]~$&"), Ok(var!(4)));
        assert_eq!(parse_and_exec("[]~$&"), Ok(var!(-1)));
        assert_eq!(
            parse_and_exec("[6.5, 6.5, 3.4]~$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            })
        );
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$&"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$&"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: AND,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
    }

    #[test]
    fn bitor_reduce() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$|"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$|"), Ok(var!(63)));
        assert_eq!(parse_and_exec("[5, 6, 7]~$|"), Ok(var!(7)));
        assert_eq!(parse_and_exec("[]~$|"), Ok(var!(0)));
        assert_eq!(
            parse_and_exec("[6.5, 6.5, 3.4]~$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[6.5, 6.5, 3.4] ~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, float))
            })
        );
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$|"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$|"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: OR,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
    }

    #[test]
    fn bitand_reduction_consumes_the_iterator_until_its_end() {
        assert_eq!(
            parse_and_exec(
                "count := mut 0
                iter := () -> (bool, int) {
                    count += 1
                    return (*count < 4, *count)
                }
                (iter$&, *count)"
            ),
            Ok(var!((0, 4)))
        );
    }

    #[test]
    fn bitor_reduction_consumes_the_iterator_until_its_end() {
        assert_eq!(
            parse_and_exec(
                "count := mut 0
                iter := () -> (bool, int) {
                    count += 1
                    return (*count < 4, -*count)
                }
                (iter$|, *count)"
            ),
            Ok(var!((-1, 4)))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script).map(|code| code.exec().unwrap())
    }
}
