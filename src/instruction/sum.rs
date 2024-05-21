use super::{
    array_repeat::ArrayRepeat,
    local_variable::{LocalVariable, LocalVariables},
    Add, Exec, ExecResult, Instruction, InstructionWithStr, Multiply, Recreate,
};
use crate::{
    variable::{Array, ReturnType, Type, Typed, Variable},
    Error, ExecError, Interpreter,
};
use duplicate::duplicate_item;
use std::sync::Arc;

pub fn create_sum(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.as_type() == [Type::Int].into() => {
            Ok(IntSum::calc(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.as_type() == [Type::Float].into() =>
        {
            Ok(FloatSum::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat
                .value
                .return_type()
                .matches(&(Type::Int | Type::Float)) =>
        {
            let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat);
            Ok(Multiply::create_from_instructions(
                value.instruction,
                len.instruction,
            ))
        }
        Instruction::Array(array)
            if array.var_type == [Type::Int].into() || array.var_type == [Type::Float].into() =>
        {
            Ok(array
                .instructions
                .iter()
                .cloned()
                .map(|iws| iws.instruction)
                .reduce(|acc, curr| Add::create_from_instructions(acc, curr))
                .unwrap())
        }
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Int].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(IntSum { array }.into())
        }
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Float].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(FloatSum { array }.into())
        }
        instruction @ Instruction::Other(_) if instruction.return_type() == [Type::Int].into() => {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(IntSum { array }.into())
        }

        instruction @ Instruction::Other(_)
            if instruction.return_type() == [Type::Float].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(FloatSum { array }.into())
        }
        ins => Err(Error::CannotSum(array.str, ins.return_type())),
    }
}

#[derive(Debug)]
pub struct IntSum {
    array: InstructionWithStr,
}

impl IntSum {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_int().unwrap()).sum();
        Variable::Int(sum)
    }
}

impl ReturnType for IntSum {
    fn return_type(&self) -> Type {
        [Type::Int].into()
    }
}

#[derive(Debug)]
pub struct FloatSum {
    array: InstructionWithStr,
}

impl FloatSum {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_float().unwrap()).sum();
        Variable::Float(sum)
    }
}

impl ReturnType for FloatSum {
    fn return_type(&self) -> Type {
        [Type::Float].into()
    }
}

#[duplicate_item(T; [IntSum]; [FloatSum])]
impl Recreate for T {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        Ok(Self { array }.into())
    }
}

#[duplicate_item(T; [IntSum]; [FloatSum])]
impl Exec for T {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(Self::calc(array).into())
    }
}
