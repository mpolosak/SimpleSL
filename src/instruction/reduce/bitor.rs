use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{local_variable::LocalVariable, Instruction, InstructionWithStr};
use crate::instruction::{Exec, ExecResult, Recreate, Xor};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};
use std::sync::Arc;

pub fn create_bitor_reduce(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.element_type() == &Type::Int => {
            Ok(BitOrReduce::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&(Type::Int)) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type == Type::Int => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| Xor::create_from_instructions(acc, curr))
            .unwrap()),
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Int].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(BitOrReduce { array }.into())
        }
        instruction @ Instruction::Other(_) if instruction.return_type() == [Type::Int].into() => {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(BitOrReduce { array }.into())
        }
        ins => Err(Error::CannotProduct(array.str, ins.return_type())),
    }
}

#[derive(Debug)]
pub struct BitOrReduce {
    pub array: InstructionWithStr,
}

impl BitOrReduce {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array
            .iter()
            .map(|var| var.as_int().unwrap())
            .fold(0, |acc, curr| acc | curr);
        Variable::Int(sum)
    }
}

impl ReturnType for BitOrReduce {
    fn return_type(&self) -> Type {
        [Type::Int].into()
    }
}

impl Recreate for BitOrReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        Ok(Self { array }.into())
    }
}

impl Exec for BitOrReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(array).into())
    }
}