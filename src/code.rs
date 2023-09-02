use crate::{
    instruction::{local_variable::LocalVariables, Exec, Instruction},
    parse::{Rule, SimpleSLParser},
    variable::{ReturnType, Type, Variable},
    Interpreter, Result,
};
use pest::Parser;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Code {
    pub(crate) instructions: Rc<[Instruction]>,
}

impl Code {
    pub fn parse(interpreter: &Interpreter, script: &str) -> Result<Self> {
        let parse = SimpleSLParser::parse(Rule::input, script)?;
        let mut local_variables = LocalVariables::new();
        let instructions = parse
            .map(|pair| Instruction::new(pair, interpreter, &mut local_variables))
            .collect::<Result<_>>()?;
        Ok(Self { instructions })
    }
    pub fn exec(&self) -> Result<Variable> {
        let mut interpreter = Interpreter::without_stdlib();
        self.exec_unscoped(&mut interpreter)
    }
    pub fn exec_unscoped(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        self.instructions
            .iter()
            .map(|instruction| instruction.exec(interpreter))
            .last()
            .unwrap_or(Ok(Variable::Void))
    }
}

impl ReturnType for Code {
    fn return_type(&self) -> Type {
        self.instructions
            .last()
            .map_or(Type::Void, ReturnType::return_type)
    }
}
