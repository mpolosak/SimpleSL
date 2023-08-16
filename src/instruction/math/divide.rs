use crate::instruction::traits::{BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction};
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

impl BinOp for Divide {
    const SYMBOL: &'static str = "/";

    fn get_lhs(&self) -> &Instruction {
        &self.dividend
    }

    fn get_rhs(&self) -> &Instruction {
        &self.divisor
    }

    fn construct(dividend: Instruction, divisor: Instruction) -> Self {
        Self { dividend, divisor }
    }
}

impl CanBeUsed for Divide {
    fn can_be_used(dividend: &Type, divisor: &Type) -> bool {
        match (dividend, divisor) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => true,
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type)) => {
                element_type.as_ref() == var_type
            }
            _ => false,
        }
    }
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
        let dividend_type = dividend.get_return_type();
        let divisor_type = divisor.get_return_type();
        if Self::can_be_used(&dividend_type, &divisor_type) {
            Self::create_from_instructions(dividend, divisor)
        } else {
            Err(Error::CannotDo2(dividend_type, Self::SYMBOL, divisor_type))
        }
    }
}

impl CreateFromInstructions for Divide {
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::divide(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroDivision),
            (dividend, divisor) => Ok(Self::construct(dividend, divisor).into()),
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
            (dividend, divisor) => panic!("Tried to calc {dividend} {} {divisor}", Self::SYMBOL),
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
