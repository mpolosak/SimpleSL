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
pub struct Divide {
    dividend: Instruction,
    divisor: Instruction,
}

impl CreateInstruction for Divide {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let dividend = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let divisor = Instruction::new(pair, interpreter, local_variables)?;
        match (dividend.get_return_type(), divisor.get_return_type()) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => {
                Self::create_from_instructions(dividend, divisor)
            }
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type))
                if element_type.as_ref() == &var_type =>
            {
                Self::create_from_instructions(dividend, divisor)
            }
            _ => Err(Error::CannotDo2(
                dividend.get_return_type(),
                "/",
                divisor.get_return_type(),
            )),
        }
    }
}

impl Divide {
    fn divide(dividend: Variable, divisor: Variable) -> Result<Variable> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(Error::ZeroDivision),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend / divisor).into()),
            (Variable::Float(dividend), Variable::Float(divisor)) => {
                Ok((dividend / divisor).into())
            }
            (Variable::Array(array, _), divisor @ (Variable::Int(_) | Variable::Float(_))) => array
                .iter()
                .cloned()
                .map(|dividend| Self::divide(dividend, divisor.clone()))
                .collect::<Result<Variable>>(),
            (dividend @ (Variable::Int(_) | Variable::Float(_)), Variable::Array(array, _)) => {
                array
                    .iter()
                    .cloned()
                    .map(|divisor| Self::divide(dividend.clone(), divisor))
                    .collect::<Result<Variable>>()
            }
            (dividend, divisor) => panic!("Tried to divide {dividend} by {divisor}"),
        }
    }
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::divide(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroDivision),
            (dividend, divisor) => Ok(Self { dividend, divisor }.into()),
        }
    }
}

impl Exec for Divide {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let dividend = self.dividend.exec(interpreter)?;
        let divisor = self.divisor.exec(interpreter)?;
        Self::divide(dividend, divisor)
    }
}

impl Recreate for Divide {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let dividend = self.dividend.recreate(local_variables, interpreter)?;
        let divisor = self.divisor.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(dividend, divisor)
    }
}

impl GetReturnType for Divide {
    fn get_return_type(&self) -> Type {
        match (
            self.dividend.get_return_type(),
            self.divisor.get_return_type(),
        ) {
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) | (var_type, _) => {
                var_type
            }
        }
    }
}

impl From<Divide> for Instruction {
    fn from(value: Divide) -> Self {
        Self::Divide(value.into())
    }
}
