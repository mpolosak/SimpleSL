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

#[derive(Clone, Debug)]
pub struct Multiply {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Multiply {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
        match (lhs.get_return_type(), rhs.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type == Type::Int.into() =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(var_type), Type::Float) | (Type::Float, Type::Array(var_type))
                if var_type == Type::Float.into() =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("*")),
        }
    }
}

impl Multiply {
    fn multiply(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs * rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs * rhs).into(),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (value, Variable::Array(array, _)) | (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::multiply(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to multiply {lhs} and {rhs}"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::multiply(lhs, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Multiply {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::multiply(lhs, rhs))
    }
}

impl Recreate for Multiply {
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

impl GetReturnType for Multiply {
    fn get_return_type(&self) -> Type {
        match (self.lhs.get_return_type(), self.rhs.get_return_type()) {
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) | (var_type, _) => {
                var_type
            }
        }
    }
}

impl From<Multiply> for Instruction {
    fn from(value: Multiply) -> Self {
        Self::Multiply(value.into())
    }
}
