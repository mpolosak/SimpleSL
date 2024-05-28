use simplesl_macros::var_type;

use crate as simplesl;
use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{Exec, ExecResult, Or, Recreate};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};

pub fn create_any(array: InstructionWithStr) -> Result<Instruction, Error> {
    match &array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&var_type!(int)) =>
        {
            Ok(Any::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&var_type!(int)) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type.matches(&var_type!(int)) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| Or::create_from_instructions(acc, curr))
            .unwrap()),
        instruction if instruction.return_type() == var_type!([int]) => Ok(Any { array }.into()),
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$||",
            expected: var_type!([int]),
            given: ins.return_type(),
        }),
    }
}

#[derive(Debug)]
pub struct Any {
    pub array: InstructionWithStr,
}

impl Any {
    fn calc(array: &Array) -> Variable {
        let sum = array.iter().any(|var| *var.as_int().unwrap() != 0);
        Variable::from(sum)
    }
}

impl ReturnType for Any {
    fn return_type(&self) -> Type {
        var_type!(int)
    }
}

impl Recreate for Any {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        if let Instruction::Variable(Variable::Array(array)) = &self.array.instruction {
            return Ok(Self::calc(array).into());
        }
        Ok(Self { array }.into())
    }
}

impl Exec for Any {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(&array).into())
    }
}
