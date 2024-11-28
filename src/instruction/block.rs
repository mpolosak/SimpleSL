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
        local_variables: &LocalVariables,
        instructions: &mut Vec<InstructionWithStr>,
    ) -> Result<(), Error> {
        let mut local_variables = local_variables.create_layer();
        let str = pair.as_str().into();
        let pairs = pair.into_inner();
        if pairs.len() == 0 {
            return Ok(());
        }
        let mut inner = Vec::<InstructionWithStr>::new();
        for pair in pairs {
            InstructionWithStr::create(pair, &mut local_variables, &mut inner)?;
        }
        let block = Block {
            instructions: inner.into(),
        };
        let iws = InstructionWithStr {
            instruction: block.into(),
            str,
        };
        instructions.push(iws);
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
        let mut local_variables = local_variables.create_layer();
        let instructions = recreate_instructions(&self.instructions, &mut local_variables)?;
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
