use super::{local_variable::LocalVariableMap, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct IfElse {
    condition: Box<Instruction>,
    if_true: Box<Instruction>,
    if_false: Box<Instruction>,
}

impl CreateInstruction for IfElse {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let condition = Instruction::new(condition_pair, variables, local_variables)?;
        if condition.get_return_type() != Type::Int {
            return Err(Error::WrongType("condition".to_owned(), Type::Int));
        }
        let true_pair = inner.next().unwrap();
        match (condition, rule) {
            (Instruction::Variable(Variable::Int(0)), Rule::if_else) => {
                let false_pair = inner.next().unwrap();
                Instruction::new(false_pair, variables, local_variables)
            }
            (Instruction::Variable(Variable::Int(0)), Rule::if_stm) => {
                Ok(Instruction::Variable(Variable::Null))
            }
            (Instruction::Variable(Variable::Int(_)), _) => {
                Instruction::new(true_pair, variables, local_variables)
            }
            (condition, Rule::if_else) => {
                let if_true = Instruction::new(true_pair, variables, local_variables)?.into();
                let false_pair = inner.next().unwrap();
                let if_false = Instruction::new(false_pair, variables, local_variables)?.into();
                Ok(Self {
                    condition: condition.into(),
                    if_true,
                    if_false,
                }
                .into())
            }
            (condition, _) => {
                let if_true = Instruction::new(true_pair, variables, local_variables)?.into();
                Ok(Self {
                    condition: condition.into(),
                    if_true,
                    if_false: Instruction::Variable(Variable::Null).into(),
                }
                .into())
            }
        }
    }
}

impl Exec for IfElse {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut crate::interpreter::VariableMap,
    ) -> Result<crate::variable::Variable, crate::error::Error> {
        if self.condition.exec(interpreter, local_variables)? == Variable::Int(0) {
            self.if_false.exec(interpreter, local_variables)
        } else {
            self.if_true.exec(interpreter, local_variables)
        }
    }
}

impl Recreate for IfElse {
    fn recreate(
        self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error> {
        let condition = self.condition.recreate(local_variables, args)?;
        match condition {
            Instruction::Variable(Variable::Int(0)) => self.if_true.recreate(local_variables, args),
            Instruction::Variable(Variable::Int(_)) => {
                self.if_false.recreate(local_variables, args)
            }
            condition => {
                let if_true = self.if_true.recreate(local_variables, args)?.into();
                let if_false = self.if_false.recreate(local_variables, args)?.into();
                Ok(Self {
                    condition: condition.into(),
                    if_true,
                    if_false,
                }
                .into())
            }
        }
    }
}

impl GetReturnType for IfElse {
    fn get_return_type(&self) -> Type {
        let true_return_type = self.if_true.get_return_type();
        let false_return_type = self.if_false.get_return_type();
        true_return_type.concat(false_return_type)
    }
}

impl From<IfElse> for Instruction {
    fn from(value: IfElse) -> Self {
        Self::IfElse(value)
    }
}
