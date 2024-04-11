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
        local_variables: &LocalVariables,
    ) -> Result<InstructionWithStr, Error> {
        let str = pair.as_str().into();
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| InstructionWithStr::new_expression(arg, local_variables))
            .collect::<Result<Arc<_>, Error>>()?;
        let instruction = Self::create_from_instructions(instructions);
        Ok(InstructionWithStr { instruction, str })
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
                instruction: Instruction::Variable(variable),
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
        Instruction::Variable(Variable::Array(
            crate::variable::Array {
                var_type,
                elements: array.into(),
            }
            .into(),
        ))
    }
}

impl Exec for Array {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.instructions)?;
        Ok(elements.into())
    }
}

impl Recreate for Array {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instructions = recreate_instructions(&self.instructions, local_variables)?;
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
