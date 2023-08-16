use crate::instruction::{local_variable::LocalVariables, Exec, Instruction, Recreate};
use crate::Error;
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct BitwiseNot {
    pub instruction: Instruction,
}

impl BitwiseNot {
    pub fn create_instruction(instruction: Instruction) -> Result<Instruction> {
        let return_type = instruction.get_return_type();
        if return_type == Type::Int || return_type == Type::Array(Type::Int.into()) {
            Ok(Self::create_from_instruction(instruction))
        } else {
            Err(Error::CannotDo("~", instruction.get_return_type()))
        }
    }
    fn bin_not(operand: Variable) -> Variable {
        match operand {
            Variable::Int(operand) => (!operand).into(),
            Variable::Array(array, _) => array.iter().cloned().map(Self::bin_not).collect(),
            operand => panic!("Tried to do ~ {operand}"),
        }
    }
    fn create_from_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Variable(Variable::Int(value)) => {
                Instruction::Variable((value == 0).into())
            }
            instruction => Self { instruction }.into(),
        }
    }
}

impl Exec for BitwiseNot {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let result = self.instruction.exec(interpreter)?;
        Ok(Self::bin_not(result))
    }
}

impl Recreate for BitwiseNot {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

impl GetReturnType for BitwiseNot {
    fn get_return_type(&self) -> Type {
        self.instruction.get_return_type()
    }
}

impl From<BitwiseNot> for Instruction {
    fn from(value: BitwiseNot) -> Self {
        Self::BinNot(value.into())
    }
}
