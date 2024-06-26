use simplesl_macros::{var, var_type};

use crate as simplesl;
use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{BitwiseAnd, Exec, ExecResult, Recreate};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};

pub fn create_bitand_reduce(array: InstructionWithStr) -> Result<Instruction, Error> {
    match &array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&var_type!(int)) =>
        {
            Ok(BitAndReduce::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&(var_type!(int))) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type == var_type!(int) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| BitwiseAnd::create_from_instructions(acc, curr))
            .unwrap()),
        instruction if instruction.return_type().matches(&var_type!([int])) => {
            Ok(BitAndReduce { array }.into())
        }
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$&",
            expected: var_type!([int]),
            given: ins.return_type(),
        }),
    }
}

#[derive(Debug)]
pub struct BitAndReduce {
    pub array: InstructionWithStr,
}

impl BitAndReduce {
    fn calc(array: &Array) -> Variable {
        let sum = array
            .iter()
            .map(|var| var.as_int().unwrap())
            .fold(!0, |acc, curr| acc & curr);
        var!(sum)
    }
}

impl ReturnType for BitAndReduce {
    fn return_type(&self) -> Type {
        var_type!(int)
    }
}

impl Recreate for BitAndReduce {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        if let Instruction::Variable(Variable::Array(array)) = &self.array.instruction {
            return Ok(Self::calc(array).into());
        }
        Ok(Self { array }.into())
    }
}

impl Exec for BitAndReduce {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(&array).into())
    }
}
