use crate::instruction::traits::PrefixOp;
use crate::instruction::Instruction;
use crate::variable::Type;

#[derive(Debug)]
pub struct Not {
    pub instruction: Instruction,
}

impl PrefixOp for Not {
    const SYMBOL: &'static str = "!";

    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn construct(instruction: Instruction) -> Self {
        Self { instruction }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&(Type::Int | Type::Array((Type::Int).into())))
    }

    fn calc_int(num: i64) -> i64 {
        (num == 0) as i64
    }
}

impl From<Not> for Instruction {
    fn from(value: Not) -> Self {
        Self::Not(value.into())
    }
}
