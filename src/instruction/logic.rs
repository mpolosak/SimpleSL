mod and;
mod or;
use crate::prefixOp;
pub use {and::And, or::Or};

prefixOp!(Not, "!", int, |num| i64::from(num == 0));
