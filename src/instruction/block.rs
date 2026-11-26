use super::{Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    errors::ErrorWithSpan,
    instruction::{BaseInstruction, parsing_context::ParsingContext, recreate_instructions},
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Block {
    pub instructions: Box<[Instruction]>,
}

impl BaseInstruction for Block {}

impl Block {
    pub fn create_instruction(
        pair: Pair<Rule>,
        parsing_context: &ParsingContext,
        errors: &mut Vec<ErrorWithSpan>,
    ) -> Instruction {
        let mut local_variables = parsing_context.create_layer();
        let instructions = local_variables.create_instructions(pair.into_inner(), errors);
        Self { instructions }.into()
    }
}

impl Exec for Block {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let mut interpreter = interpreter.create_layer();
        let mut result = Variable::none();
        for instruction in &self.instructions {
            result = instruction.exec(&mut interpreter)?;
        }
        Ok(result)
    }
}

impl Recreate for Block {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let mut local_variables = local_variables.create_layer();
        let instructions = recreate_instructions(&self.instructions, &mut local_variables);
        Self { instructions }.into()
    }
}

impl ReturnType for Block {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or_else(Type::none, ReturnType::return_type)
    }
}
