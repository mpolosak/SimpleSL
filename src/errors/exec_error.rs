use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ExecError {
    IndexToBig,
    NegativeIndex,
    NegativeLength,
    NegativeExponent,
    ZeroDivision,
    ZeroModulo,
    OverflowShift,
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexToBig => write!(f, "index must be lower than array size"),
            Self::NegativeIndex => write!(f, "cannot index with negative value"),
            Self::NegativeLength => write!(f, "length of an array cannot be negative"),
            Self::NegativeExponent => write!(f, "int value cannot be rised to a negative power"),
            Self::ZeroDivision => {
                write!(f, "Cannot divide by 0")
            }
            Self::ZeroModulo => {
                write!(f, "Cannot calculate the remainder with a divisor of 0")
            }
            Self::OverflowShift => {
                write!(f, "Cannot shift with overflow")
            }
        }
    }
}

impl std::error::Error for ExecError {}
