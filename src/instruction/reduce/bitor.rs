use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{Exec, ExecResult, Recreate, Xor};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};

pub fn create_bitor_reduce(array: InstructionWithStr) -> Result<Instruction, Error> {
    match &array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&Type::Int) =>
        {
            Ok(BitOrReduce::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&(Type::Int)) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type.matches(&Type::Int) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| Xor::create_from_instructions(acc, curr))
            .unwrap()),
        instruction if instruction.return_type().matches(&[Type::Int].into()) => {
            Ok(BitOrReduce { array }.into())
        }
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$||",
            expected: [Type::Int].into(),
            given: ins.return_type(),
        }),
    }
}

#[derive(Debug)]
pub struct BitOrReduce {
    pub array: InstructionWithStr,
}

impl BitOrReduce {
    fn calc(array: &Array) -> Variable {
        let sum = array
            .iter()
            .map(|var| var.as_int().unwrap())
            .fold(0, |acc, curr| acc | curr);
        Variable::Int(sum)
    }
}

impl ReturnType for BitOrReduce {
    fn return_type(&self) -> Type {
        Type::Int.into()
    }
}

impl Recreate for BitOrReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        if let Instruction::Variable(Variable::Array(array)) = &self.array.instruction {
            return Ok(Self::calc(array).into());
        }
        Ok(Self { array }.into())
    }
}

impl Exec for BitOrReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(&array).into())
    }
}
