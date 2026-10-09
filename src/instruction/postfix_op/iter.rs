use crate::{
    self as simplesl, Code, Error, Interpreter,
    function::Function,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref ITER: Arc<Function> = Code::parse(
        &Interpreter::with_stdlib(),
        "(array: [int], default: int) -> () -> (bool, int) {
            i := mut -1;
            len := std.len(array)
            return () -> (bool, int) {
                i+=1;
                if *i < len {
                    return (true, array[*i])
                }
                return (false, default)
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
    pub fn create_instruction(lhs: InstructionWithStr) -> Result<Instruction, Error> {
        let op = UnaryOperator::Iter;
        let lhs_type = lhs.return_type();
        if !lhs_type.matches(&var_type!([any])) {
            return Err(Error::IncorectUnaryOperatorOperand {
                ins: lhs.str,
                op,
                expected: var_type!([any]),
                given: lhs_type,
            });
        }
        Ok(Self(lhs.instruction).into())
    }
}

impl BaseInstruction for Iter {}

impl Exec for Iter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let var = self.0.exec(interpreter)?;
        let element_type = var.as_type().element_type().unwrap();
        let default = Variable::of_type(&element_type).unwrap_or(Variable::Void);
        let result = ITER
            .exec_with_args(&[var, default])
            .unwrap()
            .into_function()
            .unwrap();
        let mut result = Arc::unwrap_or_clone(result);
        result.return_type = var_type!((bool, element_type));
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
        let element_type = instruction_return_type.element_type().unwrap();
        var_type!(() -> (bool, element_type))
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
            parse_and_exec("45~"),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "45".into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!(int)
            })
        );
        assert_eq!(
            parse_and_exec(r#""abc"~"#),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: r#""abc""#.into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!(string)
            })
        );
        assert_eq!(
            parse_and_exec(
                "x := ()->[int]|string {
                    return [45, 15]
                }
                x()~"
            ),
            Err(Error::IncorectUnaryOperatorOperand {
                ins: "x ()".into(),
                op: OP,
                expected: var_type!([any]),
                given: var_type!([int] | string)
            })
        );
        assert_eq!(
            parse_and_exec(
                "x:=[45, 5]~;
                (x(), x(), x())"
            ),
            Ok(var!(((true, 45), (true, 5), (false, 0))))
        );
        assert_eq!(
            parse_and_exec(
                "x:=[45, [5.5], true]~;
                (x(), x(), x())"
            ),
            Ok(var!(((true, 45), (true, [5.5]), (true, true))))
        );
        let result = parse_and_exec(r#"[45, 5.5, "abc"]~"#).unwrap();
        assert_eq!(result.as_type(), var_type!(()->(bool, int|float|string)))
    }

    fn parse_and_exec(script: &str) -> Result<Variable, Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
