use crate::instruction::local_variable::LocalVariables;
use crate::instruction::traits::{BaseInstruction, PrefixOp};
use crate::instruction::{Instruction, Recreate};
use crate::interpreter::Interpreter;
use crate::variable::Type;
use crate::Result;

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

impl Recreate for BitwiseNot {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        PrefixOp::recreate(self, local_variables, interpreter)
    }
}

impl BaseInstruction for BitwiseNot {}
