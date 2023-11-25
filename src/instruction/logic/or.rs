use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::Typed;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct Or {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Or {
    const SYMBOL: &'static str = "||";

    fn lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for Or {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Int, Type::Int)
            | (Type::EmptyArray, Type::Int | Type::Float | Type::String)
            | (Type::Int | Type::Float | Type::String, Type::EmptyArray) => true,
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type)) => {
                var_type.as_ref() == &Type::Int
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for Or {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::or(lhs, rhs).into())
            }
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value == 0 =>
            {
                Ok(instruction)
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Or {
    fn or(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (value, Variable::Int(0)) | (Variable::Int(0), value) => value,
            (Variable::Int(_), Variable::Int(_)) => Variable::Int(1),
            (Variable::Array(array), Variable::Int(_))
            | (Variable::Int(_), Variable::Array(array)) => std::iter::repeat(Variable::Int(1))
                .take(array.len())
                .collect(),
            (lhs, rhs) => panic!("Tried {lhs} {} {rhs} which is imposible", Self::SYMBOL),
        }
    }
}

impl Exec for Or {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::or(lhs, rhs))
    }
}

impl ReturnType for Or {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs.return_type(), self.rhs.return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl BaseInstruction for Or {}
