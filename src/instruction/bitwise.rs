use crate::variable::Type;
mod bitwise_and;
mod bitwise_not;
mod bitwise_or;
mod lshift;
mod rshift;
mod xor;
pub use {
    bitwise_and::BitwiseAnd, bitwise_not::BitwiseNot, bitwise_or::BitwiseOr, lshift::LShift,
    rshift::RShift, xor::Xor,
};

fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    match (lhs, rhs) {
        (Type::Int, Type::Int) => true,
        (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
            if var_type.as_ref() == &Type::Int =>
        {
            true
        }
        _ => false,
    }
}
