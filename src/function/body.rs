use crate::{
    ExecError, instruction::InstructionWithStr, interpreter::Interpreter, variable::Variable,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Body {
    Lang(Arc<[InstructionWithStr]>),
    Native(fn(&mut Interpreter) -> Result<Variable, ExecError>),
}
