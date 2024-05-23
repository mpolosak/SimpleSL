use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{local_variable::LocalVariable, Instruction, InstructionWithStr};
use crate::instruction::{And, Exec, ExecResult, Recreate};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};
use std::sync::Arc;

pub fn create_all(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.element_type() == &Type::Int => {
            Ok(All::calc(array).into())
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
            .reduce(|acc, curr| And::create_from_instructions(acc, curr))
            .unwrap()),
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Int].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(All { array }.into())
        }
        instruction @ Instruction::Other(_) if instruction.return_type() == [Type::Int].into() => {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(All { array }.into())
        }
        ins => Err(Error::CannotProduct(array.str, ins.return_type())),
    }
}

#[derive(Debug)]
pub struct All {
    pub array: InstructionWithStr,
}

impl All {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().all(|var| *var.as_int().unwrap() != 0);
        Variable::from(sum)
    }
}

impl ReturnType for All {
    fn return_type(&self) -> Type {
        [Type::Int].into()
    }
}

impl Recreate for All {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        Ok(Self { array }.into())
    }
}

impl Exec for All {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(array).into())
    }
}
