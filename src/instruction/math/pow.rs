use crate::instruction::traits::BinOp;
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
pub struct Pow {
    base: Instruction,
    exp: Instruction,
}

impl BinOp for Pow {
    const SYMBOL: &'static str = "**";

    fn get_lhs(&self) -> &Instruction {
        &self.base
    }

    fn get_rhs(&self) -> &Instruction {
        &self.exp
    }
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
        match (base.get_return_type(), exp.get_return_type()) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Self::create_from_instructions(base, exp)
            }
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type))
                if element_type.as_ref() == &var_type =>
            {
                Self::create_from_instructions(base, exp)
            }
            (base_return_type, exp_return_type) => Err(Error::CannotDo2(
                base_return_type,
                Self::SYMBOL,
                exp_return_type,
            )),
        }
    }
}

impl Pow {
    fn pow(base: Variable, exp: Variable) -> Result<Variable> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(Error::CannotBeNegative("exponent")),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => Ok(array),
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
            (base, exp) => panic!("Tried to calc {base} {} {exp}", Self::SYMBOL),
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
    fn get_return_type(&self) -> Type {
        match (self.base.get_return_type(), self.exp.get_return_type()) {
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) | (var_type, _) => {
                var_type
            }
        }
    }
}

impl From<Pow> for Instruction {
    fn from(value: Pow) -> Self {
        Self::Pow(value.into())
    }
}
