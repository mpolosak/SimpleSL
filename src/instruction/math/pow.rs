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
pub struct Pow {
    base: Instruction,
    exp: Instruction,
}

impl CreateInstruction for Pow {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let base = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let exp = Instruction::new(pair, interpreter, local_variables)?;
        match (base.get_return_type(), exp.get_return_type()) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => {
                Self::create_from_instructions(base, exp)
            }
            _ => Err(Error::OperandsMustBeBothIntOrBothFloat("**")),
        }
    }
}

impl Pow {
    fn create_from_instructions(base: Instruction, exp: Instruction) -> Result<Instruction, Error> {
        match (base, exp) {
            (_, Instruction::Variable(Variable::Int(exp))) if exp < 0 => {
                Err(Error::CannotBeNegative("exponent"))
            }
            (
                Instruction::Variable(Variable::Int(base)),
                Instruction::Variable(Variable::Int(exp)),
            ) => Ok(Instruction::Variable(base.pow(exp as u32).into())),
            (
                Instruction::Variable(Variable::Float(base)),
                Instruction::Variable(Variable::Float(exp)),
            ) => Ok(Instruction::Variable((base.powf(exp)).into())),
            (base, exp) => Ok(Self { base, exp }.into()),
        }
    }
}

impl Exec for Pow {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let base = self.base.exec(interpreter)?;
        let exp = self.exp.exec(interpreter)?;
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(Error::CannotBeNegative("exponent")),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            _ => panic!(),
        }
    }
}

impl Recreate for Pow {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let base = self.base.recreate(local_variables, interpreter)?;
        let exp = self.exp.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(base, exp)
    }
}

impl GetReturnType for Pow {
    fn get_return_type(&self) -> Type {
        self.base.get_return_type()
    }
}

impl From<Pow> for Instruction {
    fn from(value: Pow) -> Self {
        Self::Pow(value.into())
    }
}
