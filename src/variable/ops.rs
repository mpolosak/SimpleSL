use super::{Array, Type, Typed, Variable};
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct ImposibleAddiction {
    pub lhs: Variable,
    pub rhs: Variable,
}

impl Add for Variable {
    type Output = Result<Variable, ImposibleAddiction>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
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
                .map(|element| element + value.clone())
                .collect(),
            (value, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|element| value.clone() + element)
                .collect(),
            (lhs, rhs) => Err(ImposibleAddiction { lhs, rhs }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::variable::{ops::ImposibleAddiction, Variable};

    #[test]
    fn add() {
        assert_eq!(Variable::Int(5) + Variable::Int(2), Ok(Variable::Int(7)));
        assert_eq!(
            Variable::Float(5.5) + Variable::Float(2.3),
            Ok(Variable::Float(7.8))
        );
        assert_eq!(
            Variable::String("aa".into()) + Variable::String("B".into()),
            Ok(Variable::String("aaB".into()))
        );
        assert_eq!(
            Variable::from([Variable::Int(5), Variable::Int(5), Variable::Int(6)])
                + [Variable::Int(5)].into(),
            Ok([
                Variable::Int(5),
                Variable::Int(5),
                Variable::Int(6),
                Variable::Int(5)
            ]
            .into())
        );
        assert_eq!(Variable::from([]) + [].into(), Ok([].into()));
        assert_eq!(
            Variable::from([
                Variable::Int(5),
                Variable::Float(5.5),
                Variable::String("4".into())
            ]) + [].into(),
            Ok(Variable::from_str(r#"[5, 5.5, "4"]"#).unwrap())
        );
        assert_eq!(
            Variable::from([Variable::Int(4), Variable::Int(5), Variable::Int(6)])
                + Variable::Int(5),
            Ok([Variable::Int(9), Variable::Int(10), Variable::Int(11)].into())
        );
        assert_eq!(
            Variable::from_str("[4.5, 5.7, 6.0]").unwrap() + Variable::from(3.3),
            Ok(Variable::from_str("[7.8, 9.0, 9.3]").unwrap())
        );
        assert_eq!(
            Variable::from("7") + [Variable::from("a"), Variable::from("aaa")].into(),
            Ok([
                Variable::String("7a".into()),
                Variable::String("7aaa".into())
            ]
            .into())
        );
        assert_eq!(
            Variable::from_str(r#"["a", "aaa"]"#).unwrap() + Variable::from("3"),
            Ok(Variable::from_str(r#"["a3", "aaa3"]"#).unwrap())
        );
        assert_eq!(Variable::from([]) + 5i64.into(), Ok([].into()));
        assert_eq!(Variable::from([]) + 4.5.into(), Ok([].into()));
        assert_eq!(Variable::from([]) + "".into(), Ok([].into()));
        assert_eq!(
            Variable::Int(4) + Variable::Float(4.5),
            Err(ImposibleAddiction {
                lhs: 4i64.into(),
                rhs: 4.5.into()
            })
        );
        assert_eq!(
            Variable::from("4") + 4.5.into(),
            Err(ImposibleAddiction {
                lhs: "4".into(),
                rhs: 4.5.into()
            })
        );
        assert_eq!(
            Variable::from("4") + 4i64.into(),
            Err(ImposibleAddiction {
                lhs: "4".into(),
                rhs: 4i64.into(),
            })
        );
        assert_eq!(
            Variable::from([Variable::Int(4)]) + 4.5.into(),
            Err(ImposibleAddiction {
                lhs: 4i64.into(),
                rhs: 4.5.into()
            })
        );
        assert_eq!(
            Variable::from([Variable::Int(4), Variable::Float(5.5)]) + 4.5.into(),
            Err(ImposibleAddiction {
                lhs: 4i64.into(),
                rhs: 4.5.into()
            })
        )
    }
}
