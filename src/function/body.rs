use crate::{
    instruction::{Exec, Instruction},
    interpreter::Interpreter,
    variable::Variable,
    Result,
};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum Body {
    Lang(Rc<[Instruction]>),
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
