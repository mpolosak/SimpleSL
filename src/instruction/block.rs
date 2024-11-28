use super::{
    local_variable::LocalVariables, recreate_instructions, Exec, ExecResult, Instruction,
    InstructionWithStr, Recreate,
};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Block {
    pub instructions: Arc<[InstructionWithStr]>,
}

impl Block {
    pub fn create(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let str = pair.as_str().into();
        let pairs = pair.into_inner();
        if pairs.len() == 0 {
            return Ok(());
        }
        local_variables.new_layer();
        let mut inner = Vec::<InstructionWithStr>::new();
        for pair in pairs {
            InstructionWithStr::create(pair, local_variables, &mut inner)?;
        }
        let block = Block {
            instructions: inner.into(),
        };
        let iws = InstructionWithStr {
            instruction: block.into(),
            str,
        };
        instructions.push(iws);
        local_variables.drop_layer();
        Ok(())
    }
}

impl Exec for Block {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        interpreter.push_layer();
        let result = interpreter
            .exec(&self.instructions)?
            .last()
            .cloned()
            .unwrap_or(Variable::Void);
        interpreter.pop_layer();
        Ok(result)
    }
}

impl Recreate for Block {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        local_variables.new_layer();
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
        local_variables.drop_layer();
        Ok(Self { instructions }.into())
    }
}

impl ReturnType for Block {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}
