use crate::instruction::traits::PrefixOp;
use crate::instruction::Instruction;
use crate::variable::Type;

#[derive(Debug)]
pub struct UnaryMinus {
    pub instruction: Instruction,
}

impl PrefixOp for UnaryMinus {
    const SYMBOL: &'static str = "-";

    fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }

    fn construct(instruction: Instruction) -> Self {
        Self { instruction }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&(Type::Int | Type::Float | Type::Array((Type::Float | Type::Int).into())))
    }

    fn calc_int(num: i64) -> i64 {
        -num
    }

    fn calc_float(num: f64) -> f64 {
        -num
    }
}

impl From<UnaryMinus> for Instruction {
    fn from(value: UnaryMinus) -> Self {
        Self::UnaryMinus(value.into())
    }
}
