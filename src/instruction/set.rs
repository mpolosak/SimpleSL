use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    instruction::pattern::Pattern, interpreter::Interpreter, variable::{ReturnType, Type}, Error, ExecError
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Set {
    pub pattern: Pattern,
    pub instruction: InstructionWithStr,
}

impl Set {
    pub fn new(
        pattern: Pattern,
        instruction: InstructionWithStr,
        local_variables: &mut LocalVariables,
    ) -> Self {
        local_variables.insert(pattern.ident.clone(), (&instruction.instruction).into());
        Self { pattern, instruction }
    }

    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let pattern_pair = inner.next().unwrap();
        let pair = inner.next().unwrap();
        let instruction = InstructionWithStr::new(pair, local_variables)?;
        let var_type = instruction.return_type();
        let pattern = Pattern::create_instruction(pattern_pair, local_variables, &var_type);
        if !pattern.is_matched(&var_type) {
            panic!("pattern not matched")
        }
        Ok(Self::new(pattern, instruction, local_variables).into())
    }
}

impl Exec for Set {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        interpreter.insert(self.pattern.ident.clone(), result.clone());
        Ok(result)
    }
}

impl Recreate for Set {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        local_variables.insert(self.pattern.ident.clone(), (&instruction.instruction).into());
        Ok(Self {
            pattern: self.pattern.clone(),
            instruction,
        }
        .into())
    }
}

impl ReturnType for Set {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}
