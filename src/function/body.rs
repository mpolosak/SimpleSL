use crate::{ExecError, instruction::Instruction, variable::Variable};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Body {
    Lang(Arc<[Instruction]>),
    Native(fn(&[Variable]) -> Result<Variable, ExecError>),
}
