use super::{
    bin_op::*,
    prefix_op::{BitwiseNot, Not, UnaryMinus},
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
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type_float(lhs, rhs)
    }
}

pub fn return_type_float(lhs: Type, rhs: Type) -> Type {
    if (lhs.matches(&[Type::Int].into()) && rhs == Type::Int)
        || (rhs.matches(&[Type::Int].into()) && lhs == Type::Int)
    {
        return [Type::Int].into();
    }
    if lhs.matches(&[Type::Float].into()) || rhs.matches(&[Type::Float].into()) {
        return [Type::Float].into();
    }
    if Type::from([Type::Int]).matches(&lhs) || Type::from([Type::Int]).matches(&rhs) {
        return [Type::Int] | Type::Int;
    }
    if Type::from([Type::Float]).matches(&lhs) || Type::from([Type::Float]).matches(&rhs) {
        return [Type::Float] | Type::Float;
    }
    if lhs == Type::Int {
        return Type::Int;
    }
    Type::Float
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
        return_type_int(lhs, rhs)
    }
}

pub fn return_type_int(lhs: Type, rhs: Type) -> Type {
    if lhs.matches(&[Type::Any].into()) || rhs.matches(&[Type::Any].into()) {
        return [Type::Int].into();
    }
    if Type::Array(Type::Never.into()).matches(&lhs)
        || Type::Array(Type::Never.into()).matches(&rhs)
    {
        return [Type::Int] | Type::Int;
    }
    Type::Int
}

impl ReturnType for Equal {
    fn return_type(&self) -> Type {
        Type::Int
    }
}

#[cfg(test)]
mod tests {
    use crate::variable::Type;

    #[test]
    fn return_type_int() {
        use crate::instruction::return_type::return_type_int;
        assert_eq!(return_type_int(Type::Int, Type::Int), Type::Int);
        assert_eq!(return_type_int(Type::Float, Type::Float), Type::Int);
        assert_eq!(
            return_type_int([Type::Int].into(), Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            return_type_int([Type::Float].into(), Type::Float),
            [Type::Int].into()
        );
        assert_eq!(
            return_type_int(Type::Float, [Type::Float].into()),
            [Type::Int].into()
        );
        assert_eq!(
            return_type_int(Type::Int, [Type::Int] | Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            return_type_int(Type::Float, [Type::Float] | Type::Float),
            [Type::Int] | Type::Int
        );
    }

    #[test]
    fn return_type_float() {
        use crate::instruction::return_type::return_type_float;
        assert_eq!(return_type_float(Type::Int, Type::Int), Type::Int);
        assert_eq!(return_type_float(Type::Float, Type::Float), Type::Float);
        assert_eq!(
            return_type_float([Type::Int].into(), Type::Int),
            [Type::Int].into()
        );
        assert_eq!(
            return_type_float(Type::Int, [Type::Int].into()),
            [Type::Int].into()
        );
        assert_eq!(
            return_type_float([Type::Float].into(), Type::Float),
            [Type::Float].into()
        );
        assert_eq!(
            return_type_float(Type::Float, [Type::Float].into()),
            [Type::Float].into()
        );
        assert_eq!(
            return_type_float(Type::Int, [Type::Int] | Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            return_type_float([Type::Int] | Type::Int, Type::Int),
            [Type::Int] | Type::Int
        );
        assert_eq!(
            return_type_float(Type::Float, [Type::Float] | Type::Float),
            [Type::Float] | Type::Float
        );
        assert_eq!(
            return_type_float([Type::Float] | Type::Float, Type::Float),
            [Type::Float] | Type::Float
        );
    }
}
