mod equal;
mod greater;
mod greater_or_equal;
mod lower;
mod lower_or_equal;

use crate::variable::Type;
pub use {
    equal::Equal, greater::Greater, greater_or_equal::GreaterOrEqual, lower::Lower,
    lower_or_equal::LowerOrEqual,
};

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
    use crate::{instruction::comp::return_type, variable::Type};

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
