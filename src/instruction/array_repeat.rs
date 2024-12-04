use super::recreate_instructions;
use super::{
    local_variable::LocalVariables, Exec, ExecResult, Instruction, InstructionWithStr, Recreate,
};
use crate as simplesl;
use crate::instruction::control_flow::if_else::return_type;
use crate::variable::Typed;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type},
    Error, ExecError,
};
use pest::iterators::Pair;
use simplesl_macros::{var, var_type};
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ArrayRepeat {
    pub value: Arc<[InstructionWithStr]>,
    pub len: Arc<[InstructionWithStr]>,
}

impl ArrayRepeat {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();

        let pair = inner.next().unwrap();
        let mut value = Vec::<InstructionWithStr>::new();
        InstructionWithStr::create(pair, local_variables, &mut value)?;
        let value = value.into();

        let pair = inner.next().unwrap();
        let len_str = pair.as_str().into();
        let mut len = Vec::<InstructionWithStr>::new();
        InstructionWithStr::create(pair, local_variables, &mut len)?;
        let len: Arc<[InstructionWithStr]> = len.into();
        let len_type = local_variables.result.as_ref().unwrap().as_type();
        if !len_type.matches(&Type::Int) {
            return Err(Error::WrongLengthType(len_str));
        }
        Ok(Self { value, len }.into())
    }
}

impl Exec for ArrayRepeat {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        interpreter.exec_all(&self.value)?;
        let value = interpreter.result().unwrap().clone();
        interpreter.exec_all(&self.len)?;
        let len = interpreter.result().unwrap().clone().into_int().unwrap();
        if len < 0 {
            return Err(ExecError::NegativeLength.into());
        }
        Ok(var!([value; len]))
    }
}

impl Recreate for ArrayRepeat {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let value = recreate_instructions(&self.value, local_variables)?;
        let len = recreate_instructions(&self.len, local_variables)?;
        Ok(Self { value, len }.into())
    }
}

impl ReturnType for ArrayRepeat {
    fn return_type(&self) -> Type {
        let element_type = return_type(&self.value);
        var_type!([element_type])
    }
}
