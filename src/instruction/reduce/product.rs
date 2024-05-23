pub use self::product_float_product::FloatProduct;
pub use self::product_int_product::IntProduct;
use crate::instruction::Pow;
use crate::instruction::{
    array_repeat::ArrayRepeat, local_variable::LocalVariable, Instruction, InstructionWithStr,
    Multiply,
};
use crate::{
    variable::{Array, ReturnType, Type, Typed, Variable},
    Error,
};
use duplicate::duplicate_item;
use std::sync::Arc;

pub fn create_product(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) if array.element_type() == &Type::Int => {
            Ok(IntProduct::calc(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.as_type() == [Type::Float].into() =>
        {
            Ok(FloatProduct::calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat
                .value
                .return_type()
                .matches(&(Type::Int | Type::Float)) =>
        {
            let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat);
            Pow::create_from_instructions(value.instruction, len.instruction).map_err(Error::from)
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
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Int].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(IntProduct { array }.into())
        }
        instruction @ Instruction::LocalVariable(_, LocalVariable::Other(_))
            if instruction.return_type() == [Type::Float].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(FloatProduct { array }.into())
        }
        instruction @ Instruction::Other(_) if instruction.return_type() == [Type::Int].into() => {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(IntProduct { array }.into())
        }
        instruction @ Instruction::Other(_)
            if instruction.return_type() == [Type::Float].into() =>
        {
            let array = InstructionWithStr {
                instruction,
                str: array.str,
            };
            Ok(FloatProduct { array }.into())
        }
        ins => Err(Error::CannotProduct(array.str, ins.return_type())),
    }
}

#[duplicate_item(T R; [IntProduct] [Int]; [FloatProduct] [Float])]
mod product {
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

impl IntProduct {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_int().unwrap()).product();
        Variable::Int(sum)
    }
}

impl FloatProduct {
    fn calc(array: Arc<Array>) -> Variable {
        let sum = array.iter().map(|var| var.as_float().unwrap()).product();
        Variable::Float(sum)
    }
}
