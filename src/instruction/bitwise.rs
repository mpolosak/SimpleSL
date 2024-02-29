mod and;
mod lshift;
mod or;
mod rshift;
mod xor;
use super::macros::prefixOp;
pub use {and::BitwiseAnd, lshift::LShift, or::BitwiseOr, rshift::RShift, xor::Xor};
prefixOp!(BitwiseNot, "~", int, |num: i64| !num);
