use crate::{ExecError, instruction::Instruction, interpreter::Interpreter, variable::Variable};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Body {
    Lang(Arc<[Instruction]>),
    Native(fn(&mut Interpreter) -> Result<Variable, ExecError>),
}
