use crate::instruction::{Add, Instruction};
use crate::variable::Array;
use crate::variable::{ReturnType, Type, Variable};

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
            if Type::Array(Type::Never.into()).matches(&lhs) {
                return lhs;
            }
            return rhs;
        };
        let Some(rhs_element) = rhs.element_type() else {
            if Type::Array(Type::Never.into()).matches(&rhs) {
                return rhs;
            }
            return lhs;
        };
        [lhs_element | rhs_element].into()
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
        instruction::bin_op::Add,
        variable::{Type, Variable},
        Code, Error, Interpreter,
    };

    #[test]
    fn test_add_operator() {
        assert_eq!(parse_and_exec("4+4"), Ok(Variable::Int(8)));
        assert_eq!(parse_and_exec("4.5+0.5"), Ok(Variable::Float(5.0)));
        assert_eq!(
            parse_and_exec(r#""aa" + "B""#),
            Ok(Variable::String("aaB".into()))
        );
        assert_eq!(
            parse_and_exec("[5, 5, 6] + [5]"),
            parse_and_exec("[5, 5, 6, 5]")
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5, 6] + 5"#),
            parse_and_exec("[9, 10, 11]")
        );
        assert_eq!(
            parse_and_exec(r#"[4.5, 5.7, 6.0] + 3.3"#),
            parse_and_exec("[7.8, 9.0, 9.3]")
        );
        assert_eq!(
            parse_and_exec(r#""7" + ["a", "aaa"]"#),
            parse_and_exec(r#"["7a", "7aaa"]"#)
        );
        assert_eq!(
            parse_and_exec(r#"["a", "aaa"]+"3""#),
            parse_and_exec(r#"["a3", "aaa3"]"#)
        );
        assert_eq!(
            parse_and_exec("4+4.5"),
            Err(Error::CannotDo2(Type::Int, "+", Type::Float))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4.5"#),
            Err(Error::CannotDo2(Type::String, "+", Type::Float))
        );
        assert_eq!(
            parse_and_exec(r#""4"+4"#),
            Err(Error::CannotDo2(Type::String, "+", Type::Int))
        );
        assert_eq!(
            parse_and_exec(r#"[4]+4.5"#),
            Err(Error::CannotDo2([Type::Int].into(), "+", Type::Float))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#),
            Err(Error::CannotDo2(
                [Type::Int | Type::Float].into(),
                "+",
                Type::Float
            ))
        )
    }

    #[test]
    fn return_type() {
        // int
        assert_eq!(Add::return_type(Type::Int, Type::Int), Type::Int);
        assert_eq!(
            Add::return_type(Type::Int, [Type::Int].into()),
            [Type::Int].into()
        );
        assert_eq!(
            Add::return_type([Type::Int].into(), Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            Add::return_type([Type::Int].into(), [Type::Int].into()),
            [Type::Int].into()
        );
        assert_eq!(
            Add::return_type([Type::Int].into(), [Type::Int] | Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            Add::return_type([Type::Int] | Type::Int, [Type::Int].into()),
            [Type::Int] | Type::Int
        );
        // float
        assert_eq!(Add::return_type(Type::Float, Type::Float), Type::Float);
        assert_eq!(
            Add::return_type(Type::Float, [Type::Float].into()),
            [Type::Float].into()
        );
        assert_eq!(
            Add::return_type([Type::Float].into(), Type::Float),
            [Type::Float].into()
        );
        assert_eq!(
            Add::return_type([Type::Float].into(), [Type::Float].into()),
            [Type::Float].into()
        );
        assert_eq!(
            Add::return_type([Type::Float].into(), [Type::Float] | Type::Float),
            [Type::Float] | Type::Float
        );
        assert_eq!(
            Add::return_type([Type::Float] | Type::Float, [Type::Float].into()),
            [Type::Float] | Type::Float
        );
        // string
        assert_eq!(Add::return_type(Type::String, Type::String), Type::String);
        assert_eq!(
            Add::return_type(Type::String, [Type::String].into()),
            [Type::String].into()
        );
        assert_eq!(
            Add::return_type([Type::String].into(), Type::String),
            [Type::String].into()
        );
        assert_eq!(
            Add::return_type([Type::String].into(), [Type::String].into()),
            [Type::String].into()
        );
        assert_eq!(
            Add::return_type([Type::String].into(), [Type::String] | Type::String),
            [Type::String] | Type::String
        );
        assert_eq!(
            Add::return_type([Type::String] | Type::String, [Type::String].into()),
            [Type::String] | Type::String
        );
        // array + array
        assert_eq!(
            Add::return_type([Type::Int].into(), [Type::Float].into()),
            [Type::Float | Type::Int].into()
        );
        assert_eq!(
            Add::return_type([Type::Int].into(), [Type::Any].into()),
            [Type::Any].into()
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
