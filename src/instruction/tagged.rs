use crate::{
    Error,
    instruction::{
        BaseInstruction, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
        local_variable::LocalVariables,
    },
    interpreter::Interpreter,
    variable::{ReturnType, TaggedType, TaggedValue, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug)]
pub struct Tagged {
    tag: Arc<str>,
    payload: Instruction,
}

impl Tagged {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
        errors: &mut Vec<Error>,
    ) -> Instruction {
        let mut inner = pair.into_inner();
        let tag: Arc<str> = inner.next().unwrap().as_str().into();
        let payload = inner.next().map(|pair| {
            InstructionWithStr::new_expression(pair, local_variables, errors).instruction
        });
        if let Some(payload) = payload {
            Self { tag, payload }.into()
        } else {
            Variable::from(TaggedValue::without_payload(tag)).into()
        }
    }
}

impl BaseInstruction for Tagged {}

impl Exec for Tagged {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let payload = self.payload.exec(interpreter)?;
        Ok(TaggedValue::with_payload(self.tag.clone(), payload).into())
    }
}

impl Recreate for Tagged {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        Self {
            tag: self.tag.clone(),
            payload: self.payload.recreate(local_variables),
        }
        .into()
    }
}

impl ReturnType for Tagged {
    fn return_type(&self) -> Type {
        TaggedType::with_payload(self.tag.clone(), self.payload.return_type()).into()
    }
}
