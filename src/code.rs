use crate::{
    instruction::{local_variable::LocalVariables, Exec, ExecStop, InstructionWithStr},
    variable::{ReturnType, Type, Variable},
    Error, ExecError, Interpreter,
};
use pest::Parser;
use simplesl_parser::{Rule, SimpleSLParser};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Code {
    pub(crate) instructions: Arc<[InstructionWithStr]>,
}

impl Code {
    pub fn parse(interpreter: &Interpreter, script: &str) -> Result<Self, Error> {
        let parse = SimpleSLParser::parse(Rule::input, script)?;
        let mut local_variables = LocalVariables::new(interpreter);
        let mut instructions = Vec::<InstructionWithStr>::new();
        for pair in parse {
            InstructionWithStr::create(pair, &mut local_variables, &mut instructions)?;
        }
        let instructions = instructions
            .iter()
            .map(|iws| iws.recreate(&mut local_variables))
            .collect::<Result<_, ExecError>>()?;
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
            .transpose()
        {
            Ok(_) => Ok(interpreter.result().unwrap_or(&Variable::Void).clone()),
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
            let _ = Code::parse(&Interpreter::without_stdlib(), &s).and_then(|code| Ok(code.exec()?));
        }
    }
}
