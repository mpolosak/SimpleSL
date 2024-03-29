use super::{
    local_variable::LocalVariables,
    traits::{BaseInstruction, ExecResult},
    CreateInstruction, Exec, Instruction, Recreate,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct ArrayRepeat {
    value: Instruction,
    len: Instruction,
}

impl CreateInstruction for ArrayRepeat {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let value =
            Instruction::new_expression(inner.next().unwrap(), interpreter, local_variables)?;
        let len = Instruction::new_expression(inner.next().unwrap(), interpreter, local_variables)?;
        if len.return_type() != Type::Int {
            return Err(Error::WrongType("len".into(), Type::Int));
        }
        Self::create_from_instructions(value, len)
    }
}

impl ArrayRepeat {
    fn create_from_instructions(value: Instruction, len: Instruction) -> Result<Instruction> {
        match (value, len) {
            (_, Instruction::Variable(Variable::Int(len))) if len < 0 => {
                Err(Error::CannotBeNegative("len"))
            }
            (Instruction::Variable(value), Instruction::Variable(Variable::Int(len))) => {
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
            return Err(Error::CannotBeNegative("len").into());
        }
        Ok(std::iter::repeat(value).take(len as usize).collect())
    }
}

impl Recreate for ArrayRepeat {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let value = self.value.recreate(local_variables, interpreter)?;
        let len = self.len.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(value, len)
    }
}

impl ReturnType for ArrayRepeat {
    fn return_type(&self) -> Type {
        let element_type = self.value.return_type();
        [element_type].into()
    }
}

impl BaseInstruction for ArrayRepeat {}
