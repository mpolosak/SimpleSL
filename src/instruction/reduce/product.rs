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
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
}

#[derive(Debug)]
pub struct Product {
    iterator: Instruction,
}

impl Product {
    pub fn create(iterator: InstructionWithStr) -> Result<Instruction, Error> {
        let return_type = iterator.return_type();
        if !return_type.matches(&ACCEPTED_TYPE) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: iterator.str,
                op: UnaryOperator::Product,
                expected: ACCEPTED_TYPE.clone(),
                given: return_type,
            });
        }
        Ok(Self {
            iterator: iterator.instruction,
        }
        .into())
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
            let tuple = iterator.exec(&[])?.into_tuple().unwrap();
            if let Variable::Bool(false) = tuple[0] {
                break;
            }
            result = result * tuple[1].clone();
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
        self.iterator.return_type().iter_element().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, Code, Error, Interpreter, instruction::reduce::product::ACCEPTED_TYPE,
        unary_operator::UnaryOperator, variable::Variable,
    };
    use simplesl_macros::{var, var_type};
    const OP: UnaryOperator = UnaryOperator::Product;

    #[test]
    fn product() {
        assert_eq!(
            parse_and_exec("[45, 76, 15]$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "[45, 76, 15]".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!([int])
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"$*"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec("x:= () -> int {return 5;} x$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> int)
            })
        );
        assert_eq!(
            parse_and_exec("x:= (a: int) -> (bool, int) {return (true, a);} x$*"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x".into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!((int) -> (bool, int))
            })
        );
        assert_eq!(parse_and_exec("[45, 16, 3]~$*"), Ok(var!(2160)));
        assert_eq!(parse_and_exec("[5.5, 6.5, 7.4]~$*"), Ok(var!(264.55)));
        assert_eq!(
            parse_and_exec(r#"["a", "6.5", "$"]~$*"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#"["a", "6.5", "$"] ~"#.into(),
                op: OP,
                expected: ACCEPTED_TYPE.clone(),
                given: var_type!(() -> (bool, string))
            })
        );
        assert_eq!(
            parse_and_exec("[45, 16.5, 3]~$*"),
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
                x()$*"
            ),
            Ok(var!(32400))
        );
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float){
                    return [4.5, 1.6, 4.5]~;
                }
                x()$*"
            ),
            Ok(var!(32.4))
        );
    }

    #[test]
    fn product_empty_iterators_preserve_their_element_type() {
        assert_eq!(
            parse_and_exec("x:=() -> (bool, int) { return (false, 0) } x$*"),
            Ok(var!(1))
        );
        assert_eq!(
            parse_and_exec("x:=() -> (bool, float) { return (false, 0.0) } x$*"),
            Ok(var!(1.0))
        );
    }

    #[test]
    fn product_empty_union_iterator_uses_its_concrete_type() {
        assert_eq!(
            parse_and_exec(
                "x:=() -> ()->(bool, int)|() -> (bool, float) {
                    return () -> (bool, float) { return (false, 0.0); };
                }
                x()$*"
            ),
            Ok(var!(1.0))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
