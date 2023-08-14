use std::rc::Rc;

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

#[derive(Debug)]
pub struct Subtract {
    minuend: Instruction,
    subtrahend: Instruction,
}

impl CreateInstruction for Subtract {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let minuend = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let subtrahend = Instruction::new(pair, interpreter, local_variables)?;
        let minuend_return_type = minuend.get_return_type();
        let subtrahend_return_type = subtrahend.get_return_type();
        match (
            minuend_return_type.as_ref(),
            subtrahend_return_type.as_ref(),
        ) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Ok(Self::create_from_instructions(minuend, subtrahend))
            }
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type))
                if element_type.as_ref() == var_type =>
            {
                Ok(Self::create_from_instructions(minuend, subtrahend))
            }
            _ => Err(Error::CannotDo2(
                minuend_return_type,
                "-",
                subtrahend_return_type,
            )),
        }
    }
}

impl Subtract {
    fn subtract(minuend: Variable, subtrahend: Variable) -> Variable {
        match (minuend, subtrahend) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs - rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs - rhs).into(),
            (array, _) | (_, array) if array.get_type().as_ref() == &Type::EmptyArray => array,
            (minuend, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|subtrahend| Self::subtract(minuend.clone(), subtrahend))
                .collect(),
            (Variable::Array(array, _), subtrahend) => array
                .iter()
                .cloned()
                .map(|minuend| Self::subtract(minuend, subtrahend.clone()))
                .collect(),
            (minuend, subtrahend) => panic!("Tried to subtract {minuend} from {subtrahend}"),
        }
    }
    fn create_from_instructions(minuend: Instruction, subtrahend: Instruction) -> Instruction {
        match (minuend, subtrahend) {
            (Instruction::Variable(minuend), Instruction::Variable(rhs)) => {
                Self::subtract(minuend, rhs).into()
            }
            (minuend, subtrahend) => Self {
                minuend,
                subtrahend,
            }
            .into(),
        }
    }
}

impl Exec for Subtract {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let minuend = self.minuend.exec(interpreter)?;
        let subtrahend = self.subtrahend.exec(interpreter)?;
        Ok(Self::subtract(minuend, subtrahend))
    }
}

impl Recreate for Subtract {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let minuend = self.minuend.recreate(local_variables, interpreter)?;
        let subtrahend = self.subtrahend.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instructions(minuend, subtrahend))
    }
}

impl GetReturnType for Subtract {
    fn get_return_type(&self) -> Rc<Type> {
        match (
            self.minuend.get_return_type(),
            self.subtrahend.get_return_type(),
        ) {
            (var_type, _) | (_, var_type)
                if matches!(var_type.as_ref(), Type::Array(_) | Type::EmptyArray) =>
            {
                var_type
            }
            (var_type, _) => var_type,
        }
    }
}

impl From<Subtract> for Instruction {
    fn from(value: Subtract) -> Self {
        Self::Subtract(value.into())
    }
}
