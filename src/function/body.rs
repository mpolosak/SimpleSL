use crate::{instruction::Instruction, interpreter::Interpreter, variable::Variable, ExecError};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) enum Body {
    Lang(Rc<[Instruction]>),
    Native(fn(&mut Interpreter) -> Result<Variable, ExecError>),
}
