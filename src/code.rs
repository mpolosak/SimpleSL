use crate::{
    Error, ExecError, Interpreter,
    instruction::{
        Exec, ExecStop, Instruction, local_variable::LocalVariables, recreate_instructions,
    },
    variable::{ReturnType, Type, Variable},
};
use pest::Parser;
use simplesl_parser::{Rule, SimpleSLParser};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Code {
    pub(crate) instructions: Arc<[Instruction]>,
}

impl Code {
    pub fn parse(interpreter: &Interpreter, script: &str) -> Result<Self, Vec<Error>> {
        let parse = match SimpleSLParser::parse(Rule::input, script) {
            Ok(parse) => parse,
            Err(error) => return Err([error.into()].into()),
        };
        let mut local_variables = LocalVariables::new(interpreter);
        let mut errors = Vec::new();
        let instructions: Box<[Instruction]> = parse
            .map(|pair| Instruction::new(pair, &mut local_variables, &mut errors))
            .collect();
        if !errors.is_empty() {
            return Err(errors);
        }
        let mut local_variables = LocalVariables::new(interpreter);
        let instructions = recreate_instructions(&instructions, &mut local_variables).into();
        Ok(Self { instructions })
    }
    pub fn exec(&self) -> Result<Variable, ExecError> {
        let mut interpreter = Interpreter::without_stdlib();
        self.exec_unscoped(&mut interpreter)
    }
    pub fn exec_unscoped(&self, interpreter: &mut Interpreter) -> Result<Variable, ExecError> {
        match self
            .instructions
            .iter()
            .map(|instruction| instruction.exec(interpreter))
            .last()
            .unwrap_or(Ok(Variable::Void))
        {
            Ok(var) => Ok(var),
            Err(ExecStop::Error(err)) => Err(err),
            Err(_) => unreachable!("Return statement outside of function body"),
        }
    }
}

impl ReturnType for Code {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Interpreter};
    use proptest::prelude::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Code>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Code>();
    }

    proptest! {
        #[test]
        fn code_doesnt_crash(s in "\\PC*"){
            let _ = Code::parse(&Interpreter::without_stdlib(), &s).map(|code| code.exec());
        }
    }
}
