mod add;
mod divide;
mod modulo;
mod multiply;
mod pow;
mod subtract;
use crate::prefixOp;
use std::ops::Neg;
pub use {
    add::Add, divide::Divide, modulo::Modulo, multiply::Multiply, pow::Pow, subtract::Subtract,
};

fn neg<T: Neg<Output = T>>(num: T) -> T {
    -num
}

prefixOp!(UnaryMinus, "-", num, neg);
