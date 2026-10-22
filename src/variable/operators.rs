use std::ops::Add;
use crate::variable::{Array, Variable};

impl Add for Variable{
    type Output = Variable;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => value1.wrapping_add(value2).into(),
            (Variable::Float(value1), Variable::Float(value2)) => (value1 + value2).into(),
            (Variable::String(value1), Variable::String(value2)) => {
                format!("{value1}{value2}").into()
            }
            (Variable::Array(lhs), Variable::Array(rhs)) => {Array::concat(lhs, rhs).into()}
            (lhs, rhs) => panic!("Tried to do {lhs} + {rhs} which is imposible"),
        }
    }
}

#[cfg(test)]
mod tests {
    use simplesl_macros::var;
    use crate as simplesl;
    use crate::variable::Variable;

    #[test]
    fn add_int_to_int() {
        assert_eq!(Variable::Int(4) + Variable::Int(5), Variable::Int(9))
    }

    #[test]
    fn add_float_to_float() {
        assert_eq!(Variable::Float(4.5) + Variable::Float(5.0), Variable::Float(9.5))
    }

    #[test]
    fn add_string_to_string() {
        assert_eq!(Variable::String("Hello".into()) + Variable::String(" World!".into()), Variable::String("Hello World!".into()))
    }

    #[test]
    fn add_array_to_array() {
        assert_eq!(var!([4, true]) + var!([4.5]), var!([4, true, 4.5]))
    }

    #[test]
    #[should_panic]
    fn add_int_to_float() {
        let _ = Variable::Int(4) + Variable::Float(5.5);
    }
}