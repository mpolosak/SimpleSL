use crate::instruction::local_variable::LocalVariables;
use crate::instruction::traits::{BaseInstruction, PrefixOp};
use crate::instruction::{Exec, Instruction, Recreate};
use crate::interpreter::Interpreter;
use crate::variable::{ReturnType, Type, Variable};
use crate::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
lazy_static! {
    static ref ACCEPTED_TYPE: Type = Type::from_str("int|float|[int|float]").unwrap();
}
#[derive(Debug)]
pub struct UnaryMinus {
    pub instruction: Instruction,
}

impl PrefixOp for UnaryMinus {
    const SYMBOL: &'static str = "-";

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
        -num
    }

    fn calc_float(num: f64) -> f64 {
        -num
    }
}

impl Exec for UnaryMinus {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        PrefixOp::exec(self, interpreter)
    }
}

impl Recreate for UnaryMinus {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        PrefixOp::recreate(self, local_variables, interpreter)
    }
}

impl ReturnType for UnaryMinus {
    fn return_type(&self) -> Type {
        PrefixOp::get_return_type(self)
    }
}

impl BaseInstruction for UnaryMinus {}
