mod bitwise;
mod equal;
mod filter;
mod logic;
mod map;
mod math;
mod ord;
mod reduce;
mod shift;
use crate::{
    instruction::{
        traits::{CanBeUsed, ToResult},
        Instruction,
    },
    variable::ReturnType,
    Error,
};
use duplicate::duplicate_item;
pub use reduce::*;

#[duplicate_item(T; [BitwiseAnd]; [BitwiseOr]; [Xor]; [Equal]; [Filter]; [Map]; [And]; [Or];
    [Add]; [Subtract]; [Multiply]; [Divide]; [Modulo]; [Pow]; [Greater]; [GreaterOrEqual];
    [Lower]; [LowerOrEqual]; [LShift]; [RShift]
)]
#[derive(Debug)]
pub struct T {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

#[duplicate_item(T op; [BitwiseAnd] [&]; [BitwiseOr] [|]; [Xor] [^]; [Equal] [==]; [Filter] [?];
    [Map] [@]; [And] [&&]; [Or] [||]; [Add] [+]; [Subtract] [-]; [Multiply] [*]; [Divide] [/];
    [Modulo] [%]; [Pow] [**]; [Greater] [>]; [GreaterOrEqual] [>=]; [Lower] [<];
    [LowerOrEqual] [<=]; [LShift] [<<]; [RShift] [>>]
)]
impl T {
    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        Self::create_from_instructions(lhs, rhs).to_result()
    }
}

#[duplicate_item(T; [Filter]; [Map])]
impl T {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        Self { lhs, rhs }.into()
    }
}
