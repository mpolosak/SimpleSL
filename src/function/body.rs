use crate::{
    instruction::InstructionWithStr, interpreter::Interpreter, variable::Variable, ExecError,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) enum Body {
    Lang(Arc<[InstructionWithStr]>),
    Native(fn(&mut Interpreter) -> Result<Variable, ExecError>),
}
