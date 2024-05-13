use super::{
    local_variable::LocalVariables, traits::ExecResult, Exec, Instruction, InstructionWithStr,
    Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct ArrayRepeat {
    pub value: InstructionWithStr,
    pub len: InstructionWithStr,
}

impl ArrayRepeat {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let value = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
        let len = InstructionWithStr::new_expression(inner.next().unwrap(), local_variables)?;
        if len.return_type() != Type::Int {
            return Err(Error::WrongLengthType(len.str));
        }
        Self::create_from_instructions(value, len).map_err(Error::from)
    }

    fn create_from_instructions(
        value: InstructionWithStr,
        len: InstructionWithStr,
    ) -> Result<Instruction, ExecError> {
        match (value, len) {
            (
                _,
                InstructionWithStr {
                    instruction: Instruction::Variable(Variable::Int(len)),
                    ..
                },
            ) if len < 0 => Err(ExecError::NegativeLength),
            (
                InstructionWithStr {
                    instruction: Instruction::Variable(value),
                    ..
                },
                InstructionWithStr {
                    instruction: Instruction::Variable(Variable::Int(len)),
                    ..
                },
            ) => {
                let variable: Variable = std::iter::repeat(value).take(len as usize).collect();
                Ok(variable.into())
            }
            (value, len) => Ok(Self { value, len }.into()),
        }
    }
}

impl Exec for ArrayRepeat {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let value = self.value.exec(interpreter)?;
        let Variable::Int(len) = self.len.exec(interpreter)? else {
            panic!()
        };
        if len < 0 {
            return Err(ExecError::NegativeLength.into());
        }
        Ok(std::iter::repeat(value).take(len as usize).collect())
    }
}

impl Recreate for ArrayRepeat {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let value = self.value.recreate(local_variables)?;
        let len = self.len.recreate(local_variables)?;
        Self::create_from_instructions(value, len)
    }
}

impl ReturnType for ArrayRepeat {
    fn return_type(&self) -> Type {
        let element_type = self.value.return_type();
        [element_type].into()
    }
}

impl From<ArrayRepeat> for Instruction {
    fn from(value: ArrayRepeat) -> Self {
        Self::ArrayRepeat(value.into())
    }
}
