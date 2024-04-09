use super::{
    local_variable::LocalVariables,
    recreate_instructions,
    traits::{Exec, ExecResult, Recreate},
    CreateInstruction, Instruction, InstructionWithStr,
};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::sync::Arc;

#[derive(Debug)]
pub struct Array {
    pub instructions: Arc<[InstructionWithStr]>,
    pub var_type: Type,
}

impl CreateInstruction for Array {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| InstructionWithStr::new_expression(arg, interpreter, local_variables))
            .collect::<Result<Arc<_>, Error>>()?;
        Ok(Self::create_from_instructions(instructions))
    }
}
impl Array {
    fn create_from_instructions(instructions: Arc<[InstructionWithStr]>) -> Instruction {
        let var_type = instructions
            .iter()
            .map(ReturnType::return_type)
            .reduce(Type::concat)
            .map_or(Type::EmptyArray, |element_type| [element_type].into());
        let mut array = Vec::new();
        for instruction in &*instructions {
            let InstructionWithStr {
                instruction: Instruction::Variable(_, variable),
                ..
            } = instruction
            else {
                return Self {
                    instructions,
                    var_type,
                }
                .into();
            };
            array.push(variable.clone());
        }

        Variable::Array(
            crate::variable::Array {
                var_type,
                elements: array.into(),
            }
            .into(),
        )
        .into()
    }
}

impl Exec for Array {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.instructions)?;
        Ok(elements.into())
    }
}

impl Recreate for Array {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.instructions, local_variables, interpreter)?;
        Ok(Self::create_from_instructions(instructions))
    }
}

impl ReturnType for Array {
    fn return_type(&self) -> Type {
        self.var_type.clone()
    }
}

impl From<Array> for Instruction {
    fn from(value: Array) -> Self {
        Self::Array(value.into())
    }
}
