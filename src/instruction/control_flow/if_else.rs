use crate::instruction::{
    local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct IfElse {
    condition: Instruction,
    if_true: Instruction,
    if_false: Instruction,
}

impl CreateInstruction for IfElse {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let condition = Instruction::new(condition_pair, interpreter, local_variables)?;
        if condition.get_return_type() != Type::Int {
            return Err(Error::WrongType("condition".into(), Type::Int));
        }
        let true_pair = inner.next().unwrap();
        match (condition, rule) {
            (Instruction::Variable(Variable::Int(0)), Rule::if_else) => {
                let false_pair = inner.next().unwrap();
                Instruction::new(false_pair, interpreter, local_variables)
            }
            (Instruction::Variable(Variable::Int(0)), Rule::if_stm) => {
                Ok(Instruction::Variable(Variable::Void))
            }
            (Instruction::Variable(Variable::Int(_)), _) => {
                Instruction::new(true_pair, interpreter, local_variables)
            }
            (condition, Rule::if_else) => {
                let if_true = Instruction::new(true_pair, interpreter, local_variables)?;
                let false_pair = inner.next().unwrap();
                let if_false = Instruction::new(false_pair, interpreter, local_variables)?;
                Ok(Self {
                    condition,
                    if_true,
                    if_false,
                }
                .into())
            }
            (condition, _) => {
                let if_true = Instruction::new(true_pair, interpreter, local_variables)?;
                Ok(Self {
                    condition,
                    if_true,
                    if_false: Instruction::Variable(Variable::Void),
                }
                .into())
            }
        }
    }
}

impl Exec for IfElse {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<crate::variable::Variable, Error> {
        if self.condition.exec(interpreter)? == Variable::Int(0) {
            self.if_false.exec(interpreter)
        } else {
            self.if_true.exec(interpreter)
        }
    }
}

impl Recreate for IfElse {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let condition = self.condition.recreate(local_variables, interpreter)?;
        match condition {
            Instruction::Variable(Variable::Int(0)) => {
                self.if_true.recreate(local_variables, interpreter)
            }
            Instruction::Variable(Variable::Int(_)) => {
                self.if_false.recreate(local_variables, interpreter)
            }
            condition => {
                let if_true = self.if_true.recreate(local_variables, interpreter)?;
                let if_false = self.if_false.recreate(local_variables, interpreter)?;
                Ok(Self {
                    condition,
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
        Self::IfElse(value.into())
    }
}
