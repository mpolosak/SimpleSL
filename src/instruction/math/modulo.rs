use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Modulo {
    dividend: Instruction,
    divisor: Instruction,
}

impl CreateInstruction for Modulo {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let dividend = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let divisor = Instruction::new(pair, interpreter, local_variables)?;
        match (dividend.get_return_type(), divisor.get_return_type()) {
            (Type::Int, Type::Int) => Self::create_from_instructions(dividend, divisor),
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type == Type::Int.into() =>
            {
                Self::create_from_instructions(dividend, divisor)
            }
            _ => Err(Error::BothOperandsMustBeInt("%")),
        }
    }
}
impl Modulo {
    fn modulo(dividend: Variable, divisor: Variable) -> Result<Variable, Error> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(Error::ZeroModulo),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend % divisor).into()),
            (Variable::Array(array, _), divisor @ Variable::Int(_)) => array
                .iter()
                .cloned()
                .map(|dividend| Self::modulo(dividend, divisor.clone()))
                .collect::<Result<Variable, Error>>(),
            (dividend @ Variable::Int(_), Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::modulo(dividend.clone(), divisor))
                .collect::<Result<Variable, Error>>(),
            (dividend, divisor) => panic!("Tried to divide {dividend} by {divisor}"),
        }
    }
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, Error> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::modulo(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroModulo),
            (dividend, divisor) => Ok(Self { dividend, divisor }.into()),
        }
    }
}

impl Exec for Modulo {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let dividend = self.dividend.exec(interpreter)?;
        let divisor = self.divisor.exec(interpreter)?;
        Self::modulo(dividend, divisor)
    }
}

impl Recreate for Modulo {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let dividend = self.dividend.recreate(local_variables, interpreter)?;
        let divisor = self.divisor.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(dividend, divisor)
    }
}

impl From<Modulo> for Instruction {
    fn from(value: Modulo) -> Self {
        Self::Modulo(value.into())
    }
}
