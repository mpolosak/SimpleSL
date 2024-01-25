use crate::{
    instruction::{local_variable::LocalVariables, Instruction},
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};

use super::{Exec, Recreate};

pub trait PrefixOp: Sized + Into<Instruction> {
    const SYMBOL: &'static str;
    fn instruction(&self) -> &Instruction;
    fn construct(instruction: Instruction) -> Self;
    fn can_be_used(var_type: &Type) -> bool;
    fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(operand) => Self::calc_int(operand).into(),
            Variable::Float(operand) => Self::calc_float(operand).into(),
            Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
            operand => panic!("Tried to {} {operand}", Self::SYMBOL),
        }
    }
    fn calc_int(num: i64) -> i64 {
        panic!("Tried to {} {num}", Self::SYMBOL)
    }
    fn calc_float(num: f64) -> f64 {
        panic!("Tried to {} {num}", Self::SYMBOL)
    }
    fn create_instruction(instruction: Instruction) -> Result<Instruction> {
        let return_type = instruction.return_type();
        if Self::can_be_used(&return_type) {
            Ok(Self::create_from_instruction(instruction))
        } else {
            Err(Error::CannotDo(Self::SYMBOL, return_type))
        }
    }
    fn create_from_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Variable(operand) => Self::calc(operand).into(),
            instruction => Self::construct(instruction).into(),
        }
    }
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instruction = self.instruction().recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

impl<T: PrefixOp> ReturnType for T {
    fn return_type(&self) -> Type {
        self.instruction().return_type()
    }
}

impl<T: PrefixOp> Exec for T {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let result = self.instruction().exec(interpreter)?;
        Ok(Self::calc(result))
    }
}
