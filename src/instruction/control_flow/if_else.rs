use crate::instruction::traits::{BaseInstruction, ExecResult};
use crate::instruction::{
    local_variable::LocalVariables, traits::MutCreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct IfElse {
    condition: Instruction,
    if_true: Instruction,
    if_false: Instruction,
}

impl MutCreateInstruction for IfElse {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let condition = Instruction::new(condition_pair, interpreter, local_variables)?;
        if condition.return_type() != Type::Int {
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
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        if self.condition.exec(interpreter)? == Variable::Int(0) {
            return self.if_false.exec(interpreter);
        }
        self.if_true.exec(interpreter)
    }
}

impl Recreate for IfElse {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let condition = self.condition.recreate(local_variables, interpreter)?;
        let Instruction::Variable(Variable::Int(condition)) = condition else {
            let if_true = self.if_true.recreate(local_variables, interpreter)?;
            let if_false = self.if_false.recreate(local_variables, interpreter)?;
            return Ok(Self {
                condition,
                if_true,
                if_false,
            }
            .into());
        };
        if condition == 0 {
            return self.if_false.recreate(local_variables, interpreter);
        }
        self.if_true.recreate(local_variables, interpreter)
    }
}

impl ReturnType for IfElse {
    fn return_type(&self) -> Type {
        let true_return_type = self.if_true.return_type();
        let false_return_type = self.if_false.return_type();
        true_return_type | false_return_type
    }
}

impl BaseInstruction for IfElse {}
