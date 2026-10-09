use super::{Exec, ExecResult, Instruction, Recreate, local_variable::LocalVariables};
use crate::{
    self as simplesl, Error,
    instruction::{BaseInstruction, recreate_instructions},
    interpreter::Interpreter,
    variable::{ReturnType, Type},
};
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Array {
    pub instructions: Box<[Instruction]>,
    pub element_type: Type,
}

impl Array {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let inner = pair.into_inner();
        let mut local_variables = local_variables.create_layer();
        let instructions = inner
            .map(|arg| Instruction::new(arg, &mut local_variables))
            .collect::<Result<Box<_>, Error>>()?;
        let element_type = instructions
            .iter()
            .map(ReturnType::return_type)
            .reduce(Type::concat)
            .unwrap_or(Type::Never);
        Ok(Self {
            instructions,
            element_type,
        }
        .into())
    }
}

impl BaseInstruction for Array {}

impl Exec for Array {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.instructions)?;
        Ok(elements.into())
    }
}

impl Recreate for Array {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let instructions = recreate_instructions(&self.instructions, local_variables);
        let mut array = Vec::new();
        for instruction in &*instructions {
            let Instruction::Variable(variable) = instruction else {
                return Self {
                    instructions,
                    element_type: self.element_type.clone(),
                }
                .into();
            };
            array.push(variable.clone());
        }
        Instruction::Variable(array.into())
    }
}

impl ReturnType for Array {
    fn return_type(&self) -> Type {
        let element_type = self.element_type.clone();
        var_type!([element_type])
    }
}
