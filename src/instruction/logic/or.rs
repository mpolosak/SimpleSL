use crate::instruction::traits::{BinOp, CanBeUsed};
use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Or {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Or {
    const SYMBOL: &'static str = "||";

    fn get_lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn get_rhs(&self) -> &Instruction {
        &self.rhs
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

impl CreateInstruction for Or {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        let lhs_type = lhs.get_return_type();
        let rhs_type = rhs.get_return_type();
        if Self::can_be_used(&lhs_type, &rhs_type) {
            Ok(Self::create_from_instructions(lhs, rhs))
        } else {
            Err(Error::CannotDo2(lhs_type, Self::SYMBOL, rhs_type))
        }
    }
}

impl Or {
    fn or(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (value @ Variable::Array(_, Type::EmptyArray), _)
            | (_, value @ Variable::Array(_, Type::EmptyArray))
            | (value, Variable::Int(0))
            | (Variable::Int(0), value) => value,
            (Variable::Int(_), Variable::Int(_)) => Variable::Int(1),
            (Variable::Array(array, _), Variable::Int(_))
            | (Variable::Int(_), Variable::Array(array, _)) => std::iter::repeat(Variable::Int(1))
                .take(array.len())
                .collect(),
            (lhs, rhs) => panic!("Tried {lhs} {} {rhs} which is imposible", Self::SYMBOL),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::or(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value == 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
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

impl Recreate for Or {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(lhs, rhs))
    }
}

impl GetReturnType for Or {
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

impl From<Or> for Instruction {
    fn from(value: Or) -> Self {
        Self::Or(value.into())
    }
}
