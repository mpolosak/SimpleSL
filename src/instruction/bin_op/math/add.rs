use crate as simplesl;
use crate::instruction::{Add, Instruction};
use crate::variable::Array;
use crate::variable::{ReturnType, Type, Variable};
use simplesl_macros::var_type;

impl Add {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => (value1 + value2).into(),
            (Variable::Float(value1), Variable::Float(value2)) => (value1 + value2).into(),
            (Variable::String(value1), Variable::String(value2)) => {
                format!("{value1}{value2}").into()
            }
            (Variable::Array(array1), Variable::Array(array2)) => {
                Array::concat(array1, array2).into()
            }
            (Variable::Array(array), rhs) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| Self::exec(lhs, rhs.clone()))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            }
            (lhs, Variable::Array(array)) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|rhs| Self::exec(lhs.clone(), rhs))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            }
            (lhs, rhs) => panic!("Tried to do {lhs} + {rhs} which is imposible"),
        }
    }

    fn return_type(lhs: Type, rhs: Type) -> Type {
        let Some(lhs_element) = lhs.element_type() else {
            if var_type!([]).matches(&lhs) {
                return lhs;
            }
            return rhs;
        };
        let Some(rhs_element) = rhs.element_type() else {
            if var_type!([]).matches(&rhs) {
                return rhs;
            }
            return lhs;
        };
        var_type!([lhs_element | rhs_element])
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
    use crate as simplesl;
    use crate::{instruction::bin_op::Add, variable::Variable, Code, Error, Interpreter};
    use simplesl_macros::{var, var_type};

    #[test]
    fn test_add_operator() {
        assert_eq!(parse_and_exec("4+4"), Ok(Variable::Int(8)));
        assert_eq!(parse_and_exec("4.5+0.5"), Ok(Variable::Float(5.0)));
        assert_eq!(parse_and_exec(r#""aa" + "B""#), Ok(var!("aaB")));
        assert_eq!(parse_and_exec("[5, 5, 6] + [5]"), Ok(var!([5, 5, 6, 5])));
        assert_eq!(parse_and_exec(r#"[4, 5, 6] + 5"#), Ok(var!([9, 10, 11])));
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] + 3.3"#),
            Ok(var!([7.8, 9.0, 9.3]))
        );
        assert_eq!(
            parse_and_exec(r#""7" + ["a", "aaa"]"#),
            Ok(var!(["7a", "7aaa"]))
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]+"3""#),
            Ok(var!(["a3", "aaa3"]))
        );
        assert_eq!(
            parse_and_exec("4+4.5"),
            Err(Error::CannotDo2(var_type!(int), "+", var_type!(float)))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4.5"#),
            Err(Error::CannotDo2(var_type!(string), "+", var_type!(float)))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4"#),
            Err(Error::CannotDo2(var_type!(string), "+", var_type!(int)))
        );
        assert_eq!(
            parse_and_exec(r#"[4]+4.5"#),
            Err(Error::CannotDo2(var_type!([int]), "+", var_type!(float)))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#),
            Err(Error::CannotDo2(
                var_type!([int | float]),
                "+",
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
        assert_eq!(
            Add::return_type(var_type!(int), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!(int)),
            var_type!([int])
        );
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!([int])),
            var_type!([int])
        );
        assert_eq!(
            Add::return_type(var_type!([int]), var_type!([int] | int)),
            var_type!([int] | int)
        );
        assert_eq!(
            Add::return_type(var_type!([int] | int), var_type!([int])),
            var_type!([int] | int)
        );
        // float
        assert_eq!(
            Add::return_type(var_type!(float), var_type!(float)),
            var_type!(float)
        );
        assert_eq!(
            Add::return_type(var_type!(float), var_type!([float])),
            var_type!([float])
        );
        assert_eq!(
            Add::return_type(var_type!([float]), var_type!(float)),
            var_type!([float])
        );
        assert_eq!(
            Add::return_type(var_type!([float]), var_type!([float])),
            var_type!([float])
        );
        assert_eq!(
            Add::return_type(var_type!([float]), var_type!([float] | float)),
            var_type!([float] | float)
        );
        assert_eq!(
            Add::return_type(var_type!([float] | float), var_type!([float])),
            var_type!([float] | float)
        );
        // string
        assert_eq!(
            Add::return_type(var_type!(string), var_type!(string)),
            var_type!(string)
        );
        assert_eq!(
            Add::return_type(var_type!(string), var_type!([string])),
            var_type!([string])
        );
        assert_eq!(
            Add::return_type(var_type!([string]), var_type!(string)),
            var_type!([string])
        );
        assert_eq!(
            Add::return_type(var_type!([string]), var_type!([string])),
            var_type!([string])
        );
        assert_eq!(
            Add::return_type(var_type!([string]), var_type!([string] | string)),
            var_type!([string] | string)
        );
        assert_eq!(
            Add::return_type(var_type!([string] | string), var_type!([string])),
            var_type!([string] | string)
        );
        // array + array
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
