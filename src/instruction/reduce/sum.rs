pub use self::sums_float_sum::*;
pub use self::sums_int_sum::*;
pub use self::sums_string_sum::*;
use crate::instruction::{
    array_repeat::ArrayRepeat, local_variable::LocalVariable, Add, Instruction, InstructionWithStr,
    Multiply,
};
use crate::{
    variable::{Array, ReturnType, Type, Typed, Variable},
    Error,
};
use duplicate::duplicate_item;
use std::sync::Arc;

pub fn create_sum(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.element_type() == &Type::Int => {
            Ok(IntSum::calc(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.as_type() == [Type::Float].into() =>
        {
            Ok(FloatSum::calc(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.as_type() == [Type::String].into() =>
        {
            Ok(StringSum::calc(array).into())
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
            if array.element_type == Type::Int
                || array.element_type == Type::Float
                || array.element_type == Type::String =>
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
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::String].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(StringSum { array }.into())
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
        instruction @ Instruction::Other(_)
            if instruction.return_type() == [Type::String].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(StringSum { array }.into())
        }
        ins => Err(Error::CannotSum(array.str, ins.return_type())),
    }
}

#[duplicate_item(T R; [IntSum] [Int]; [FloatSum] [Float]; [StringSum] [String])]
mod sums {
    use crate::{
        instruction::{
            local_variable::LocalVariables, Exec, ExecResult, Instruction, InstructionWithStr,
            Recreate,
        },
        variable::{ReturnType, Type},
        ExecError, Interpreter,
    };

    #[derive(Debug)]
    pub struct T {
        pub array: InstructionWithStr,
    }

    impl ReturnType for T {
        fn return_type(&self) -> Type {
            [Type::R].into()
        }
    }

    impl Recreate for T {
        fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
            let array = self.array.recreate(local_variables)?;
            Ok(Self { array }.into())
        }
    }

    impl Exec for T {
        fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
            let array = self.array.exec(interpreter)?.into_array().unwrap();
            Ok(Self::calc(array).into())
        }
    }
}

impl IntSum {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_int().unwrap()).sum();
        Variable::Int(sum)
    }
}

impl FloatSum {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_float().unwrap()).sum();
        Variable::Float(sum)
    }
}

impl StringSum {
    fn calc(array: Arc<Array>) -> Variable {
        let sum: String = array
            .iter()
            .map(|var| var.as_string().unwrap())
            .fold(String::new(), |acc, curr| format!("{acc}{curr}"));
        Variable::from(sum)
    }
}
