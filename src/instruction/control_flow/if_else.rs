use crate::instruction::{
    local_variable::LocalVariables, traits::ExecResult, Exec, Instruction, InstructionWithStr,
    Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

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
        let return_type = condition.return_type();
        if return_type != Type::Int {
            return Err(Error::WrongCondition(condition.str, return_type));
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
                |pair| Instruction::new(pair, local_variables),
            );
        }
        Instruction::new(true_pair, local_variables)
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
