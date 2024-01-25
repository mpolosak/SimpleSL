use crate::instruction::traits::PrefixOp;
use crate::instruction::Instruction;
use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {
    static ref ACCEPTED_TYPE: Type = Type::from_str("int|[int]").unwrap();
}

#[derive(Debug)]
pub struct Not {
    pub instruction: Instruction,
}

impl PrefixOp for Not {
    const SYMBOL: &'static str = "!";

    fn instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn construct(instruction: Instruction) -> Self {
        Self { instruction }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&ACCEPTED_TYPE)
    }

    fn calc_int(num: i64) -> i64 {
        i64::from(num == 0)
    }
}
