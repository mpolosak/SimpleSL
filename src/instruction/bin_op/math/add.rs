use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{BaseInstruction, Exec, Instruction, Recreate};
use crate::variable::{Array, ReturnType, Type, Variable};
use crate::{self as simplesl, BinOperator, Error};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    static ref ACCEPTED_TYPE: Type =
        var_type!((int, int) | (float, float) | (string, string) | ([any], [any]));
}

#[derive(Debug)]
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl Add {
    pub fn create_instruction(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_rt = lhs.return_type();
        let rhs_rt = rhs.return_type();
        if !Self::can_be_used(lhs_rt.clone(), rhs_rt.clone()) {
            return Err(Error::CannotDo2(lhs_rt, BinOperator::Add, rhs_rt));
        }
        Ok(Self { lhs, rhs }.into())
    }

    pub fn can_be_used(lhs: Type, rhs: Type) -> bool {
        var_type!((lhs, rhs)).matches(&ACCEPTED_TYPE)
    }

    pub fn calc(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => value1.wrapping_add(value2).into(),
            (Variable::Float(value1), Variable::Float(value2)) => (value1 + value2).into(),
            (Variable::String(value1), Variable::String(value2)) => {
                format!("{value1}{value2}").into()
            }
            (Variable::Array(array1), Variable::Array(array2)) => {
                Array::concat(array1, array2).into()
            }
            (lhs, rhs) => panic!("Tried to do {lhs} + {rhs} which is imposible"),
        }
    }

    pub fn return_type(lhs: Type, rhs: Type) -> Type {
        let Some(lhs_element) = lhs.element_type() else {
            return lhs;
        };
        let rhs_element = rhs.element_type().unwrap();
        var_type!([lhs_element | rhs_element])
    }
}

impl BaseInstruction for Add {}

impl Exec for Add {
    fn exec(&self, interpreter: &mut crate::Interpreter) -> crate::instruction::ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::calc(lhs, rhs))
    }
}

impl Recreate for Add {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let lhs = self.lhs.recreate(local_variables);
        let rhs = self.rhs.recreate(local_variables);
        let (Some(lhs), Some(rhs)) = (
            lhs.downcast_ref::<Variable>(),
            rhs.downcast_ref::<Variable>(),
        ) else {
            return Self { lhs, rhs }.into();
        };
        Self::calc(lhs.clone(), rhs.clone()).into()
    }
}

impl ReturnType for Add {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        Self::return_type(lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        self as simplesl, BinOperator, Code, Error, Interpreter,
        instruction::bin_op::add::Add,
        variable::Variable,
    };
    use simplesl_macros::{var, var_type};

    #[test]
    fn test_add_operator() {
        assert_eq!(parse_and_exec("4+4"), Ok(Variable::Int(8)));
        assert_eq!(parse_and_exec("4.5+0.5"), Ok(Variable::Float(5.0)));
        assert_eq!(parse_and_exec(r#""aa" + "B""#), Ok(var!("aaB")));
        assert_eq!(parse_and_exec("[5, 5, 6] + [5]"), Ok(var!([5, 5, 6, 5])));
        assert_eq!(
            parse_and_exec(r#"[4, 5, 6] + 5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] + 3.3"#),
            Err(Error::CannotDo2(
                var_type!([float]),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""7" + ["a", "aaa"]"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!([string])
            ))
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]+"3""#),
            Err(Error::CannotDo2(
                var_type!([string]),
                BinOperator::Add,
                var_type!(string)
            ))
        );
        assert_eq!(
            parse_and_exec("4+4.5"),
            Err(Error::CannotDo2(
                var_type!(int),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4.5"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4"#),
            Err(Error::CannotDo2(
                var_type!(string),
                BinOperator::Add,
                var_type!(int)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4]+4.5"#),
            Err(Error::CannotDo2(
                var_type!([int]),
                BinOperator::Add,
                var_type!(float)
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#),
            Err(Error::CannotDo2(
                var_type!([int | float]),
                BinOperator::Add,
                var_type!(float)
            ))
        )
    }

    #[test]
    fn return_type() {
        // int
        assert_eq!(
            Add::return_type(var_type!(int), var_type!(int)),
            var_type!(int)
        );
        // float
        assert_eq!(
            Add::return_type(var_type!(float), var_type!(float)),
            var_type!(float)
        );
        // string
        assert_eq!(
            Add::return_type(var_type!(string), var_type!(string)),
            var_type!(string)
        );
        // array + array
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            Add::return_type(var_type!([float]), var_type!([float])),
            var_type!([float])
        );
        assert_eq!(
            Add::return_type(var_type!([string]), var_type!([string])),
            var_type!([string])
        );
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!([float])),
            var_type!([float | int])
        );
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!([any])),
            var_type!([any])
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
