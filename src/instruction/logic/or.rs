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
pub struct Or {
    lhs: Instruction,
    rhs: Instruction,
}

impl CreateInstruction for Or {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, variables, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, variables, local_variables)?;
        match (
            lhs.get_return_type().as_ref(),
            rhs.get_return_type().as_ref(),
        ) {
            (Type::Int, Type::Int)
            | (Type::EmptyArray, Type::Int | Type::Float | Type::String)
            | (Type::Int | Type::Float | Type::String, Type::EmptyArray) => {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type.as_ref() == &Type::Int =>
            {
                Ok(Self::create_from_instructions(lhs, rhs))
            }
            _ => Err(Error::CannotDo2(
                lhs.get_return_type(),
                "||",
                rhs.get_return_type(),
            )),
        }
    }
}

impl Or {
    fn or(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (value, _) | (_, value) if value.get_type().as_ref() == &Type::EmptyArray => value,
            (value, Variable::Int(0)) | (Variable::Int(0), value) => value,
            (Variable::Int(_), Variable::Int(_)) => Variable::Int(1),
            (Variable::Array(array, _), Variable::Int(_))
            | (Variable::Int(_), Variable::Array(array, _)) => std::iter::repeat(Variable::Int(1))
                .take(array.len())
                .collect(),
            (lhs, rhs) => panic!("Tried {lhs} && {rhs} which is imposible"),
        }
    }
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::or(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value == 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Or {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::or(lhs, rhs))
    }
}

impl Recreate for Or {
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

impl GetReturnType for Or {
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

impl From<Or> for Instruction {
    fn from(value: Or) -> Self {
        Self::Or(value.into())
    }
}
