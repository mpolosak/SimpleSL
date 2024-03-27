use crate::{instruction::Instruction, interpreter::Interpreter, variable::Variable, ExecError};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) enum Body {
    Lang(Arc<[Instruction]>),
    Native(fn(&mut Interpreter) -> Result<Variable, ExecError>),
}
