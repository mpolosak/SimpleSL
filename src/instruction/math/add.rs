use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::{Array, Typed};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Add {
    const SYMBOL: &'static str = "+";

    fn lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for Add {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::String, Type::String)
            | (Type::Array(_), Type::Array(_)) => true,
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float | Type::String))
            | (var_type @ (Type::Int | Type::Float | Type::String), Type::Array(element_type)) => {
                element_type.as_ref() == var_type
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for Add {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::add(lhs, rhs).into())
            }
            (rhs, lhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Add {
    fn add(lhs: Variable, rhs: Variable) -> Variable {
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
                .map(|element| Self::add(element, value.clone()))
                .collect(),
            (value, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|element| Self::add(value.clone(), element))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for Add {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::add(lhs, rhs))
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

impl BaseInstruction for Add {}

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
        // assert_eq!(parse_and_exec(r#"[] + """#), Variable::from_str("[]"));
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
            Err(Error::CannotDo2(
                Type::Array(Type::Int.into()),
                "+",
                Type::Float
            ))
        );
        assert_eq!(
            parse_and_exec(r#"[4, 5.5]+4.5"#),
            Err(Error::CannotDo2(
                Type::Array((Type::Int | Type::Float).into()),
                "+",
                Type::Float
            ))
        )
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script).and_then(|code| code.exec())
    }
}
