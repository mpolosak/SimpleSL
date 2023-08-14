use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::variable::GetType;
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;
use std::rc::Rc;

#[derive(Debug)]
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
        let lhs_return_type = lhs.get_return_type();
        let rhs_return_type = rhs.get_return_type();
        match (lhs_return_type.as_ref(), rhs_return_type.as_ref()) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type))
                if element_type.as_ref() == var_type =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::CannotDo2(lhs_return_type, "*", rhs_return_type)),
        }
    }
}

impl Multiply {
    fn multiply(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs * rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs * rhs).into(),
            (array, _) | (_, array) if array.get_type().as_ref() == &Type::EmptyArray => array,
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
    fn get_return_type(&self) -> Rc<Type> {
        match (self.lhs.get_return_type(), self.rhs.get_return_type()) {
            (var_type, _) | (_, var_type)
                if matches!(var_type.as_ref(), Type::Array(_) | Type::EmptyArray) =>
            {
                var_type
            }
            (var_type, _) => var_type,
        }
    }
}

impl From<Multiply> for Instruction {
    fn from(value: Multiply) -> Self {
        Self::Multiply(value.into())
    }
}
