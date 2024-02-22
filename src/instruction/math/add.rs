use crate::binOpCBU;
use crate::instruction::{Exec, Instruction};
use crate::variable::{Array, Typed};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {
    static ref ACCEPTED_TYPE: Type = Type::from_str(
        "(int|[int], int|[int]) | (float|[float], float|[float]) | (string|[string], string|[string]) | ([any], [any])"
    )
    .unwrap();
}

binOpCBU!(Add, "+");

impl Add {
    fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::exec(lhs, rhs)?.into()
            }
            (rhs, lhs) => Self { lhs, rhs }.into(),
        })
    }

    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => Ok((value1 + value2).into()),
            (Variable::Float(value1), Variable::Float(value2)) => Ok((value1 + value2).into()),
            (Variable::String(value1), Variable::String(value2)) => {
                Ok(format!("{value1}{value2}").into())
            }
            (Variable::Array(array1), Variable::Array(array2)) => {
                Ok(Array::concat(array1, array2).into())
            }
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array)
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
}

impl ReturnType for Add {
    fn return_type(&self) -> Type {
        match (self.lhs.return_type(), self.rhs.return_type()) {
            (Type::Array(element_type1), Type::Array(element_type2)) => Type::Array(
                (element_type1.as_ref().clone() | element_type2.as_ref().clone()).into(),
            ),
            (_, var_type @ Type::Array(_)) | (var_type, _) => var_type,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::{
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

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
