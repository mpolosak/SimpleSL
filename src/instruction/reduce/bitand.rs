use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{local_variable::LocalVariable, Instruction, InstructionWithStr};
use crate::instruction::{BitwiseAnd, Exec, ExecResult, Recreate};
use crate::{
    variable::{Array, ReturnType, Type, Typed, Variable},
    Error,
};
use crate::{ExecError, Interpreter};
use std::sync::Arc;

pub fn create_bitand_reduce(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.as_type() == [Type::Int].into() => {
            Ok(BitAndReduce::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&(Type::Int)) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.var_type == [Type::Int].into() => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| BitwiseAnd::create_from_instructions(acc, curr))
            .unwrap()),
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Int].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(BitAndReduce { array }.into())
        }
        instruction @ Instruction::Other(_) if instruction.return_type() == [Type::Int].into() => {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(BitAndReduce { array }.into())
        }
        ins => Err(Error::CannotProduct(array.str, ins.return_type())),
    }
}

#[derive(Debug)]
pub struct BitAndReduce {
    pub array: InstructionWithStr,
}

impl BitAndReduce {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array
            .iter()
            .map(|var| var.as_int().unwrap())
            .fold(!0, |acc, curr| acc & curr);
        Variable::Int(sum)
    }
}

impl ReturnType for BitAndReduce {
    fn return_type(&self) -> Type {
        [Type::Int].into()
    }
}

impl Recreate for BitAndReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        Ok(Self { array }.into())
    }
}

impl Exec for BitAndReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(array).into())
    }
}
