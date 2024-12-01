use crate::instruction::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate::variable::Typed;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct IfElse {
    pub if_true: Arc<[InstructionWithStr]>,
    pub if_false: Arc<[InstructionWithStr]>,
}

impl IfElse {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let mut inner = pair.into_inner();
        let condition_pair = inner.next().unwrap();
        let str = condition_pair.as_str().into();
        InstructionWithStr::create(condition_pair, local_variables, instructions)?;
        let return_type = local_variables.result.as_ref().unwrap().as_type();
        if return_type != Type::Bool {
            return Err(Error::WrongCondition(str, return_type));
        }

        let true_pair = inner.next().unwrap();
        let mut if_true = Vec::<InstructionWithStr>::new();
        InstructionWithStr::create(true_pair, local_variables, &mut if_true)?;
        let if_true = if_true.into();

        let mut if_false = Vec::<InstructionWithStr>::new();
        if let Some(pair) = inner.next() {
            InstructionWithStr::create(pair, local_variables, &mut if_false)?;
        }
        let if_false = if_false.into();

        let instruction: Instruction = Self { if_true, if_false }.into();
        local_variables.result = Some((&instruction).into());
        let instruction = InstructionWithStr {
            instruction,
            str: "if else".into(),
        };
        instructions.push(instruction);
        Ok(())
    }
}

impl Exec for IfElse {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let condition = *interpreter.result().unwrap().as_bool().unwrap();
        let to_exec = if condition {
            &self.if_true
        } else {
            &self.if_false
        };
        interpreter.exec_all(to_exec)?;
        Ok(interpreter.result().cloned().unwrap_or(Variable::Void))
    }
}

impl Recreate for IfElse {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let if_true = recreate_instructions(&self.if_true, local_variables)?;
        let if_false = recreate_instructions(&self.if_false, local_variables)?;
        return Ok(Self { if_true, if_false }.into());
    }
}

impl ReturnType for IfElse {
    fn return_type(&self) -> Type {
        let true_return_type = return_type(&self.if_true);
        let false_return_type = return_type(&self.if_false);
        true_return_type | false_return_type
    }
}

pub fn return_type(instructions: &[InstructionWithStr]) -> Type {
    match instructions.last() {
        Some(InstructionWithStr {
            instruction: Instruction::ExitScope,
            ..
        }) => return_type(&instructions[..instructions.len() - 2]),
        Some(ins) => ins.return_type(),
        None => Type::Void,
    }
}
