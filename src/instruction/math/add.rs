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
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Add {
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
            | (Type::String, Type::String)
            | (Type::Array(_), Type::Array(_)) => Ok(Self::create_from_instructions(lhs, rhs)),
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type.as_ref() == &Type::Int =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(var_type), Type::Float) | (Type::Float, Type::Array(var_type))
                if var_type.as_ref() == &Type::Float =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(var_type), Type::String) | (Type::String, Type::Array(var_type))
                if var_type.as_ref() == &Type::String =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::EmptyArray, Type::Int | Type::Float | Type::String)
            | (Type::Int | Type::Float | Type::String, Type::EmptyArray) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::CannotAdd(lhs_return_type, rhs_return_type)),
        }
    }
}

impl Add {
    fn add(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(value1), Variable::Int(value2)) => (value1 + value2).into(),
            (Variable::Float(value1), Variable::Float(value2)) => (value1 + value2).into(),
            (Variable::String(value1), Variable::String(value2)) => {
                format!("{value1}{value2}").into()
            }
            (Variable::Array(array1, _), Variable::Array(array2, _)) => {
                array1.iter().chain(array2.iter()).cloned().collect()
            }
            (array, _) | (_, array) if array.get_type().as_ref() == &Type::EmptyArray => array,
            (Variable::Array(array, element_type), value)
                if element_type.as_ref() == &Type::Array(Type::String.into()) =>
            {
                array
                    .iter()
                    .cloned()
                    .map(|element| Self::add(element, value.clone()))
                    .collect()
            }
            (value, Variable::Array(array, _)) | (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::add(value.clone(), element))
                .collect(),
            (lhs, rhs) => panic!("Tried to add {lhs} and {rhs} which is imposible"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::add(lhs, rhs).into(),
            (rhs, lhs) => Self { rhs, lhs }.into(),
        }
    }
}

impl Exec for Add {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::add(lhs, rhs))
    }
}

impl Recreate for Add {
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

impl GetReturnType for Add {
    fn get_return_type(&self) -> Rc<Type> {
        match (
            self.lhs.get_return_type().as_ref(),
            self.rhs.get_return_type().as_ref(),
        ) {
            (Type::Array(element_type1), Type::Array(element_type2)) => {
                Type::Array(element_type1.concat(element_type2.as_ref()).into()).into()
            }
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) | (var_type, _) => {
                var_type.clone().into()
            }
        }
    }
}

impl From<Add> for Instruction {
    fn from(value: Add) -> Self {
        Self::Add(value.into())
    }
}
