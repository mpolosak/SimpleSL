use crate::{
    ExecError, Interpreter,
    errors::ErrorWithSpan,
    instruction::{
        Exec, ExecStop, Instruction, local_variable::LocalVariables,
        parsing_context::ParsingContext, recreate_instructions,
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
    pub fn parse(interpreter: &Interpreter, script: &str) -> Result<Self, Vec<ErrorWithSpan>> {
        let parse = match SimpleSLParser::parse(Rule::input, script) {
            Ok(parse) => parse,
            Err(error) => return Err([error.into()].into()),
        };
        let local_variables = LocalVariables::new(interpreter);
        let mut parsing_context = ParsingContext {
            source: script,
            local_variables,
        };
        let mut errors = Vec::new();
        let instructions: Box<[Instruction]> = parse
            .map(|pair| Instruction::new(pair, &mut parsing_context, &mut errors))
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
        let mut result = Variable::none();
        for instruction in self.instructions.iter() {
            match instruction.exec(interpreter) {
                Ok(var) => result = var,
                Err(ExecStop::Error(err)) => return Err(err),
                Err(_) => unreachable!("Return statement outside of function body"),
            }
        }
        Ok(result)
    }
}

impl ReturnType for Code {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or_else(Type::none, ReturnType::return_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, ExecError, Interpreter, variable::Variable};
    use proptest::prelude::*;

    #[test]
    fn execution_stops_after_runtime_error() {
        let mut interpreter = Interpreter::without_stdlib();
        let code = Code::parse(&interpreter, "x := mut 0\n1 / 0\nx = 1").unwrap();

        assert!(matches!(
            code.exec_unscoped(&mut interpreter),
            Err(ExecError::ZeroDivision)
        ));
        let Variable::Mut(value) = interpreter.get_variable("x").unwrap() else {
            panic!("x should remain mutable");
        };
        assert_eq!(*value.variable.read().unwrap(), Variable::Int(0));
    }

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
