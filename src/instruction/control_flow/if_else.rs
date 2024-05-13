use crate::instruction::InstructionWithStr;
use crate::instruction::{
    local_variable::LocalVariables, traits::ExecResult, Exec, Instruction, Recreate,
};
use crate::ExecError;
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct IfElse {
    condition: InstructionWithStr,
    if_true: InstructionWithStr,
    if_false: InstructionWithStr,
}

impl IfElse {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let condition = InstructionWithStr::new(condition_pair, local_variables)?;
        if condition.return_type() != Type::Int {
            return Err(Error::WrongType("condition".into(), Type::Int));
        }
        let true_pair = inner.next().unwrap();
        let Instruction::Variable(Variable::Int(condition)) = condition.instruction else {
            let if_true = InstructionWithStr::new(true_pair, local_variables)?;
            let if_false = inner.next().map_or_else(
                || Ok(Variable::Void.into()),
                |pair| InstructionWithStr::new(pair, local_variables),
            )?;
            return Ok(Self {
                condition,
                if_true,
                if_false,
            }
            .into());
        };
        if condition == 0 {
            return inner.next().map_or_else(
                || Ok(Variable::Void.into()),
                |pair| InstructionWithStr::new(pair, local_variables).map(|iws| iws.instruction),
            );
        }
        InstructionWithStr::new(true_pair, local_variables).map(|iws| iws.instruction)
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
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let condition = self.condition.recreate(local_variables)?;
        let Instruction::Variable(Variable::Int(condition)) = condition.instruction else {
            let if_true = self.if_true.recreate(local_variables)?;
            let if_false = self.if_false.recreate(local_variables)?;
            return Ok(Self {
                condition,
                if_true,
                if_false,
            }
            .into());
        };
        if condition == 0 {
            return self.if_false.instruction.recreate(local_variables);
        }
        self.if_true.instruction.recreate(local_variables)
    }
}

impl ReturnType for IfElse {
    fn return_type(&self) -> Type {
        let true_return_type = self.if_true.return_type();
        let false_return_type = self.if_false.return_type();
        true_return_type | false_return_type
    }
}
