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
        let mut pairs = pair.into_inner();
        if pairs.len() == 0 {
            return Ok(());
        }
        if pairs.len() == 1 {
            return InstructionWithStr::create(
                pairs.next().unwrap(),
                &mut local_variables,
                instructions,
            );
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
        let mut interpreter = interpreter.create_layer();
        Ok(interpreter
            .exec(&self.instructions)?
            .last()
            .cloned()
            .unwrap_or(Variable::Void))
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
