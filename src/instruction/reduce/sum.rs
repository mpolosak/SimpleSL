use simplesl_macros::{var, var_type};

use crate as simplesl;
use crate::instruction::local_variable::LocalVariables;
use crate::instruction::{add, multiply, Exec, ExecResult, Recreate};
use crate::instruction::{array_repeat::ArrayRepeat, Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Type, Variable},
    Error,
};
use crate::{ExecError, Interpreter};
use std::sync::Arc;

#[derive(Debug)]
pub struct Sum {
    pub array: InstructionWithStr,
}

impl Sum {
    pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
        match &array.instruction {
            Instruction::Variable(Variable::Array(array))
                if array.element_type().matches(&var_type!(int)) =>
            {
                Ok(Self::calc_int(array).into())
            }
            Instruction::Variable(Variable::Array(array))
                if array.element_type() == &var_type!(float) =>
            {
                Ok(Self::calc_float(array).into())
            }
            Instruction::Variable(Variable::Array(array))
                if array.element_type() == &var_type!(string) =>
            {
                Ok(Self::calc_string(array).into())
            }
            Instruction::ArrayRepeat(array_repeat)
                if array_repeat
                    .value
                    .return_type()
                    .matches(&var_type!(int | float)) =>
            {
                let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat.clone());
                Ok(multiply::create_from_instructions(
                    value.instruction,
                    len.instruction,
                ))
            }
            Instruction::Array(array)
                if array.element_type.matches(&var_type!(int | float | string)) =>
            {
                Ok(array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|iws| iws.instruction)
                    .reduce(|acc, curr| add::create_from_instructions(acc, curr))
                    .unwrap())
            }
            instruction
                if instruction
                    .return_type()
                    .matches(&var_type!([int] | [float] | [string])) =>
            {
                Ok(Self { array }.into())
            }
            ins => Err(Error::IncorectPostfixOperatorOperand {
                ins: array.str,
                op: "$+",
                expected: var_type!([int] | [float] | [string]),
                given: ins.return_type(),
            }),
        }
    }

    fn calc(array: &Array) -> Variable {
        match array.element_type() {
            var_type!(int) => Self::calc_int(&array),
            var_type!(float) => Self::calc_float(&array),
            var_type!(string) => Self::calc_string(&array),
            element_type => unreachable!("Tried to sum [{element_type}]"),
        }
    }

    fn calc_int(array: &Array) -> Variable {
        let sum = array.iter().map(|var| var.as_int().unwrap()).sum();
        Variable::Int(sum)
    }

    fn calc_float(array: &Array) -> Variable {
        let sum = array.iter().map(|var| var.as_float().unwrap()).sum();
        Variable::Float(sum)
    }

    fn calc_string(array: &Array) -> Variable {
        let sum: String = array
            .iter()
            .map(|var| var.as_string().unwrap())
            .fold(String::new(), |acc, curr| format!("{acc}{curr}"));
        var!(sum)
    }
}

impl Recreate for Sum {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        if let Instruction::Variable(Variable::Array(array)) = &self.array.instruction {
            return Ok(Self::calc(array).into());
        }
        Ok(Self { array }.into())
    }
}

impl Exec for Sum {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let Variable::Array(array) = self.array.exec(interpreter)? else {
            unreachable!("Tried to sum not array")
        };
        Ok(Self::calc(&array))
    }
}

impl ReturnType for Sum {
    fn return_type(&self) -> Type {
        self.array.return_type().element_type().unwrap()
    }
}
