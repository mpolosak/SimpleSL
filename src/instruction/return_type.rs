use super::{
    bitwise::{BitwiseAnd, BitwiseOr, Xor},
    logic::{And, Or},
    math::{Divide, Modulo, Multiply, Pow, Subtract},
    ord::{Greater, GreaterOrEqual, Lower, LowerOrEqual},
    prefix_op::{BitwiseNot, Not, UnaryMinus},
    shift::{LShift, RShift},
};
use crate::variable::{ReturnType, Type};
use duplicate::duplicate_item;

#[duplicate_item(T; [Not]; [BitwiseNot]; [UnaryMinus])]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

#[duplicate_item(T; [Multiply]; [Divide]; [Pow]; [Subtract])]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        match (self.lhs.return_type(), self.rhs.return_type()) {
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) => var_type,
            (Type::EmptyArray, var_type) | (var_type, Type::EmptyArray) => [var_type].into(),
            (var_type, _) => var_type,
        }
    }
}

#[duplicate_item(
    T;
    [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual];
    [BitwiseAnd]; [BitwiseOr]; [And]; [Or]; [Xor];
    [Modulo]; [LShift]; [RShift]
)]
impl ReturnType for T {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type(lhs, rhs)
    }
}

pub fn return_type(lhs: Type, rhs: Type) -> Type {
    if lhs.matches(&[Type::Any].into()) || rhs.matches(&[Type::Any].into()) {
        return [Type::Int].into();
    }
    if Type::from([Type::Never]).matches(&lhs) || Type::from([Type::Never]).matches(&rhs) {
        return [Type::Int] | Type::Int;
    }
    Type::Int
}

#[cfg(test)]
mod tests {
    use crate::{instruction::return_type::return_type, variable::Type};

    #[test]
    fn test_return_type() {
        assert_eq!(return_type(Type::Int, Type::Int), Type::Int);
        assert_eq!(return_type(Type::Float, Type::Float), Type::Int);
        assert_eq!(
            return_type([Type::Int].into(), Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            return_type([Type::Float].into(), Type::Float),
            [Type::Int].into()
        );
        assert_eq!(
            return_type(Type::Float, [Type::Float].into()),
            [Type::Int].into()
        );
        assert_eq!(
            return_type(Type::Int, [Type::Int] | Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            return_type(Type::Float, [Type::Float] | Type::Float),
            [Type::Int] | Type::Int
        );
    }
}
