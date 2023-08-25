use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct And {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for And {
    const SYMBOL: &'static str = "&&";

    fn get_lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn get_rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for And {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Int, Type::Int)
            | (Type::EmptyArray, Type::Int)
            | (Type::Int, Type::EmptyArray) => true,
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type)) => {
                var_type.as_ref() == &Type::Int
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for And {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::and(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value != 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self::construct(lhs, rhs).into(),
        })
    }
}

impl And {
    fn and(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (Variable::Int(_), Variable::Int(0)) | (Variable::Int(0), Variable::Int(_)) => {
                Variable::Int(0)
            }
            (Variable::Array(array, _), Variable::Int(0))
            | (Variable::Int(0), Variable::Array(array, _)) => std::iter::repeat(Variable::Int(0))
                .take(array.len())
                .collect(),
            (value, Variable::Int(_)) | (Variable::Int(_), value) => value,
            (lhs, rhs) => panic!("Tried {lhs} {} {rhs} which is imposible", Self::SYMBOL),
        }
    }
}

impl Exec for And {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::and(lhs, rhs))
    }
}

impl GetReturnType for And {
    fn get_return_type(&self) -> Type {
        if matches!(
            (self.lhs.get_return_type(), self.rhs.get_return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl BaseInstruction for And {}
