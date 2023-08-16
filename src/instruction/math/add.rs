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
pub struct Add {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Add {
    const SYMBOL: &'static str = "+";

    fn get_lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn get_rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for Add {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::String, Type::String)
            | (Type::Array(_), Type::Array(_)) => true,
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float | Type::String))
            | (var_type @ (Type::Int | Type::Float | Type::String), Type::Array(element_type)) => {
                element_type.as_ref() == var_type
            }
            _ => false,
        }
    }
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
        let lhs_type = lhs.get_return_type();
        let rhs_type = rhs.get_return_type();
        if Self::can_be_used(&lhs_type, &rhs_type) {
            Self::create_from_instructions(lhs, rhs)
        } else {
            Err(Error::CannotDo2(lhs_type, Self::SYMBOL, rhs_type))
        }
    }
}

impl CreateFromInstructions for Add {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::add(lhs, rhs).into())
            }
            (rhs, lhs) => Ok(Self::construct(lhs, rhs).into()),
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
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (Variable::Array(array, element_type), value)
                if element_type == Type::Array(Type::String.into()) =>
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
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
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

impl GetReturnType for Add {
    fn get_return_type(&self) -> Type {
        match (self.lhs.get_return_type(), self.rhs.get_return_type()) {
            (Type::Array(element_type1), Type::Array(element_type2)) => Type::Array(
                (element_type1.as_ref().clone() | element_type2.as_ref().clone()).into(),
            ),
            (var_type @ Type::Array(_), _) | (_, var_type @ Type::Array(_)) | (var_type, _) => {
                var_type
            }
        }
    }
}

impl From<Add> for Instruction {
    fn from(value: Add) -> Self {
        Self::Add(value.into())
    }
}
