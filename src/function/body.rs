use crate::{
    instruction::{Exec, Instruction},
    interpreter::Interpreter,
    variable::Variable,
    Result,
};

#[derive(Debug)]
pub enum Body {
    Lang(Box<[Instruction]>),
    Native(fn(&mut Interpreter) -> Result<Variable>),
}

impl Exec for Body {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        match self {
            Body::Lang(body) => interpreter.exec(body),
            Body::Native(body) => (body)(interpreter),
        }
    }
}
