use crate::instruction::traits::PrefixOp;
use crate::instruction::Instruction;
use crate::variable::Type;

#[derive(Debug)]
pub struct BitwiseNot {
    pub instruction: Instruction,
}

impl PrefixOp for BitwiseNot {
    const SYMBOL: &'static str = "~";

    fn instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn construct(instruction: Instruction) -> Self {
        Self { instruction }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&(Type::Int | [Type::Int]))
    }

    fn calc_int(num: i64) -> i64 {
        !num
    }
}
