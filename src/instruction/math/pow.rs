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
pub struct Pow {
    base: Instruction,
    exp: Instruction,
}

impl CreateInstruction for Pow {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let base = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let exp = Instruction::new(pair, interpreter, local_variables)?;
        let base_return_type = base.get_return_type();
        let exp_return_type = exp.get_return_type();
        match (base_return_type.as_ref(), exp_return_type.as_ref()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                Self::create_from_instructions(base, exp)
            }
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type.as_ref() == &Type::Int =>
            {
                Self::create_from_instructions(base, exp)
            }
            (Type::Array(var_type), Type::Float) | (Type::Float, Type::Array(var_type))
                if var_type.as_ref() == &Type::Float =>
            {
                Self::create_from_instructions(base, exp)
            }
            (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Self::create_from_instructions(base, exp)
            }
            _ => Err(Error::CannotDo2(base_return_type, "**", exp_return_type)),
        }
    }
}

impl Pow {
    fn pow(base: Variable, exp: Variable) -> Result<Variable> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(Error::CannotBeNegative("exponent")),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            (array, _) | (_, array) if array.get_type().as_ref() == &Type::EmptyArray => Ok(array),
            (value, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|element| Self::pow(value.clone(), element))
                .collect(),
            (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::pow(element, value.clone()))
                .collect(),
            (base, exp) => panic!("Tried to raise {base} to {exp} power"),
        }
    }
    fn create_from_instructions(base: Instruction, exp: Instruction) -> Result<Instruction> {
        match (base, exp) {
            (Instruction::Variable(base), Instruction::Variable(exp)) => {
                Ok(Self::pow(base, exp)?.into())
            }
            (_, Instruction::Variable(Variable::Int(exp))) if exp < 0 => {
                Err(Error::CannotBeNegative("exponent"))
            }
            (base, exp) => Ok(Self { base, exp }.into()),
        }
    }
}

impl Exec for Pow {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let base = self.base.exec(interpreter)?;
        let exp = self.exp.exec(interpreter)?;
        Pow::pow(base, exp)
    }
}

impl Recreate for Pow {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let base = self.base.recreate(local_variables, interpreter)?;
        let exp = self.exp.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(base, exp)
    }
}

impl GetReturnType for Pow {
    fn get_return_type(&self) -> Rc<Type> {
        match (self.base.get_return_type(), self.exp.get_return_type()) {
            (var_type, _) | (_, var_type)
                if matches!(var_type.as_ref(), Type::Array(_) | Type::EmptyArray) =>
            {
                var_type
            }
            (var_type, _) => var_type,
        }
    }
}

impl From<Pow> for Instruction {
    fn from(value: Pow) -> Self {
        Self::Pow(value.into())
    }
}
