mod add;
mod divide;
mod modulo;
mod multiply;
mod pow;
mod subtract;
mod unary_minus;
use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;
pub use {
    add::Add, divide::Divide, modulo::Modulo, multiply::Multiply, pow::Pow, subtract::Subtract,
    unary_minus::UnaryMinus,
};

lazy_static! {
    static ref ACCEPTED_TYPE: Type = Type::from_str(
        "(int|[int], int) | (int, [int]) | (float|[float], float) | (float, [float]) "
    )
    .unwrap();
}
