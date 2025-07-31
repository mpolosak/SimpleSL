use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum ExecError {
    #[display("index out of bounds")]
    IndexOutOfBounds,
    #[display("length of an array cannot be negative")]
    NegativeLength,
    #[display("int value cannot be rised to a negative power")]
    NegativeExponent,
    #[display("Cannot divide by 0")]
    ZeroDivision,
    #[display("Cannot calculate the remainder with a divisor of 0")]
    ZeroModulo,
    #[display("Cannot shift with overflow")]
    OverflowShift,
}

impl std::error::Error for ExecError {}
