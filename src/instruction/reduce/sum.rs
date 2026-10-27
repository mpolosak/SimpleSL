use crate::{
    self as simplesl, Error,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!(() -> (bool, int) | () -> (bool, float) | () -> (bool, string));
}

#[derive(Debug)]
pub struct Sum {
    iterator: Instruction,
}

impl Sum {
    pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
        let return_type = iterator.return_type();
        if !return_type.matches(&ACCEPTED_TYPE) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: iterator.str,
                op: UnaryOperator::Sum,
                expected: ACCEPTED_TYPE.clone(),
                given: return_type,
            });
        }
        Ok(Self {
            iterator: iterator.instruction,
        }
        .into())
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
            let tuple = iterator.exec(&[])?.into_tuple().unwrap();
            if let Variable::Bool(false) = tuple[0] {
                break;
            }
            let value = tuple[1].clone();
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
        self.iterator.return_type().iter_element().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::sum::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Sum;

    #[test]
    fn sum() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$+"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$+"), Ok(var!(64)));
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$+"), Ok(var!(19.4)));
        assert_eq!(parse_and_exec(r#"["a", "6.5", "$"]~$+"#), Ok(var!("a6.5$")));
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$+"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 16.5, 3] ~".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, int|float))
            })
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [45, 16, 45]~;
                }
                x()$+"
            ),
            Ok(var!(106))
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$+"
            ),
            Ok(var!(10.6))
        );
    }

    #[test]
    fn sum_empty_iterators_preserve_their_element_type() {
        assert_eq!(
            parse_and_exec("x:=() -> (bool, float) { return (false, 0.0) } x$+"),
            Ok(var!(0.0))
        );
        assert_eq!(
            parse_and_exec(r#"x:=() -> (bool, string) { return (false, "") } x$+"#),
            Ok(var!(""))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
