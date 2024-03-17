use crate::instruction::{bin_op::binOp, Instruction};
use crate::variable::{Array, Typed};
use crate::variable::{ReturnType, Type, Variable};

binOp!(Add, "+");

impl Add {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (rhs, lhs) => Self { lhs, rhs }.into(),
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
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(element, value.clone()))
                .collect(),
            (value, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(value.clone(), element))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} + {rhs} which is imposible"),
        }
    }

    fn return_type(lhs: Type, rhs: Type) -> Type {
        if lhs == Type::EmptyArray && !Type::EmptyArray.matches(&rhs) {
            return [rhs].into();
        }
        if rhs == Type::EmptyArray && !Type::EmptyArray.matches(&lhs) {
            return [lhs].into();
        }
        let Some(lhs_element) = lhs.element_type() else {
            if Type::EmptyArray.matches(&lhs) {
                return lhs;
            }
            return rhs;
        };
        let Some(rhs_element) = rhs.element_type() else {
            if Type::EmptyArray.matches(&rhs) {
                return rhs;
            }
            return lhs;
        };
        if lhs_element == Type::Never && rhs_element == Type::Never {
            return Type::EmptyArray;
        }
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
    use std::str::FromStr;

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
        assert_eq!(parse_and_exec("[] + []"), Variable::from_str("[]"));
        assert_eq!(
            parse_and_exec(r#"[5, 5.5, "4"] + []"#),
            parse_and_exec(r#"[5, 5.5, "4"]"#)
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
        assert_eq!(parse_and_exec("[] + 5"), Variable::from_str("[]"));
        assert_eq!(parse_and_exec("[] + 4.5"), Variable::from_str("[]"));
        assert_eq!(parse_and_exec(r#"[] + """#), Variable::from_str("[]"));
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
        assert_eq!(
            Add::return_type([Type::Int] | [Type::Float].into(), Type::EmptyArray),
            [Type::Float | Type::Int].into()
        );
        assert_eq!(
            Add::return_type(Type::EmptyArray, Type::EmptyArray),
            Type::EmptyArray
        );
        // int | float | string + empty_array
        assert_eq!(
            Add::return_type(Type::Int, Type::EmptyArray),
            [Type::Int].into()
        );
        assert_eq!(
            Add::return_type(Type::EmptyArray, Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            Add::return_type(Type::Float, Type::EmptyArray),
            [Type::Float].into()
        );
        assert_eq!(
            Add::return_type(Type::EmptyArray, Type::Float),
            [Type::Float].into()
        );
        assert_eq!(
            Add::return_type(Type::String, Type::EmptyArray),
            [Type::String].into()
        );
        assert_eq!(
            Add::return_type(Type::EmptyArray, Type::String),
            [Type::String].into()
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
