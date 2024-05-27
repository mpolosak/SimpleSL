use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{array_repeat::ArrayRepeat, Instruction, InstructionWithStr, Multiply};
use crate::instruction::{Exec, ExecResult, Pow, Recreate};
use crate::{
    variable::{Array, ReturnType, Type, Typed, Variable},
    Error,
};
use crate::{ExecError, Interpreter};
use std::sync::Arc;

#[derive(Debug)]
pub struct Product {
    pub array: InstructionWithStr,
}

impl Product {
    pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
        match &array.instruction {
            Instruction::Variable(Variable::Array(array)) if array.element_type() == &Type::Int => {
                Ok(Self::calc_int(&array).into())
            }
            Instruction::Variable(Variable::Array(array))
                if array.as_type() == [Type::Float].into() =>
            {
                Ok(Self::calc_float(&array).into())
            }
            Instruction::ArrayRepeat(array_repeat)
                if array_repeat
                    .value
                    .return_type()
                    .matches(&(Type::Int | Type::Float)) =>
            {
                let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat.clone());
                Pow::create_from_instructions(value.instruction, len.instruction)
                    .map_err(Error::from)
            }
            Instruction::Array(array)
                if array.element_type == Type::Int
                    || array.element_type == Type::Float
                    || array.element_type == Type::String =>
            {
                Ok(array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|iws| iws.instruction)
                    .reduce(|acc, curr| Multiply::create_from_instructions(acc, curr))
                    .unwrap())
            }
            instruction
                if instruction
                    .return_type()
                    .matches(&([Type::Int] | [Type::Float].into())) =>
            {
                Ok(Self { array }.into())
            }
            ins => Err(Error::CannotProduct(array.str, ins.return_type())),
        }
    }

    fn calc(array: &Array) -> Variable {
        match array.element_type() {
            Type::Int => Self::calc_int(&array),
            Type::Float => Self::calc_float(&array),
            element_type => unreachable!("Tried to calculate product of [{element_type}]"),
        }
    }

    fn calc_int(array: &Array) -> Variable {
        let product = array.iter().map(|var| var.as_int().unwrap()).product();
        Variable::Int(product)
    }

    fn calc_float(array: &Array) -> Variable {
        let product = array.iter().map(|var| var.as_float().unwrap()).product();
        Variable::Float(product)
    }
}

impl Recreate for Product {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        if let Instruction::Variable(Variable::Array(array)) = &self.array.instruction {
            return Ok(Self::calc(array).into());
        }
        Ok(Self { array }.into())
    }
}

impl Exec for Product {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let Variable::Array(array) = self.array.exec(interpreter)? else {
            unreachable!("Tried to sum not array")
        };
        Ok(Self::calc(&array))
    }
}

impl ReturnType for Product {
    fn return_type(&self) -> Type {
        self.array.return_type().element_type().unwrap()
    }
}
