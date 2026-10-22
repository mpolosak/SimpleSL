use std::ops::{Add, Div, Mul, Rem, Sub};
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

impl Sub for Variable {
    type Output = Variable;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_sub(rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs - rhs).into(),
            (lhs, rhs) => panic!("Tried to do {lhs} - {rhs} which is impossible"),
        }
    }
}

impl Mul for Variable {
    type Output = Variable;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_mul(rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs * rhs).into(),
            (lhs, rhs) => panic!("Tried to do {lhs} * {rhs}"),
        }
    }
}

impl Div for Variable {
    type Output = Variable;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_div(rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs / rhs).into(),
            (lhs, rhs) => panic!("Tried to do {lhs} / {rhs}"),
        }
    }
}

impl Rem for Variable {
    type Output = Variable;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_rem(rhs).into(),
            (lhs, rhs) => panic!("Tried to do {lhs} % {rhs}"),
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

    #[test]
    fn subtract_int_from_int() {
        assert_eq!(Variable::Int(9) - Variable::Int(5), Variable::Int(4))
    }

    #[test]
    fn subtract_float_from_float() {
        assert_eq!(Variable::Float(9.5) - Variable::Float(5.0), Variable::Float(4.5))
    }

    #[test]
    fn subtract_int_with_wrapping() {
        assert_eq!(
            Variable::Int(i64::MIN) - Variable::Int(1),
            Variable::Int(i64::MAX)
        )
    }

    #[test]
    #[should_panic]
    fn subtract_int_from_float() {
        let _ = Variable::Int(4) - Variable::Float(5.5);
    }

    #[test]
    fn multiply_int_by_int() {
        assert_eq!(Variable::Int(4) * Variable::Int(5), Variable::Int(20))
    }

    #[test]
    fn multiply_float_by_float() {
        assert_eq!(
            Variable::Float(4.5) * Variable::Float(2.0),
            Variable::Float(9.0)
        )
    }

    #[test]
    fn multiply_int_with_wrapping() {
        assert_eq!(
            Variable::Int(i64::MAX) * Variable::Int(2),
            Variable::Int(-2)
        )
    }

    #[test]
    #[should_panic]
    fn multiply_int_by_float() {
        let _ = Variable::Int(4) * Variable::Float(5.5);
    }

    #[test]
    fn divide_int_by_int() {
        assert_eq!(Variable::Int(20) / Variable::Int(5), Variable::Int(4))
    }

    #[test]
    fn divide_float_by_float() {
        assert_eq!(
            Variable::Float(9.0) / Variable::Float(2.0),
            Variable::Float(4.5)
        )
    }

    #[test]
    fn divide_int_with_wrapping() {
        assert_eq!(
            Variable::Int(i64::MIN) / Variable::Int(-1),
            Variable::Int(i64::MIN)
        )
    }

    #[test]
    #[should_panic]
    fn divide_int_by_float() {
        let _ = Variable::Int(4) / Variable::Float(2.0);
    }

    #[test]
    fn remainder_int_by_int() {
        assert_eq!(Variable::Int(17) % Variable::Int(5), Variable::Int(2))
    }

    #[test]
    fn remainder_int_with_wrapping() {
        assert_eq!(
            Variable::Int(i64::MIN) % Variable::Int(-1),
            Variable::Int(0)
        )
    }

    #[test]
    #[should_panic]
    fn remainder_int_by_float() {
        let _ = Variable::Int(4) % Variable::Float(2.0);
    }
}