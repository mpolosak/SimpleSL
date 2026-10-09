use super::{
    Exec, ExecResult, Instruction, InstructionWithStr, Recreate, local_variable::LocalVariables,
};
use crate::{
    Error,
    instruction::{BaseInstruction, recreate_instructions},
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;

#[derive(Debug)]
pub struct Tuple {
    pub elements: Box<[Instruction]>,
}

impl BaseInstruction for Tuple {}

impl Tuple {
    pub fn create_instruction(
        pair: Pair<Rule>,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error> {
        let elements = pair
            .into_inner()
            .map(|pair| Ok(InstructionWithStr::new_expression(pair, local_variables)?.instruction))
            .collect::<Result<Box<[Instruction]>, Error>>()?;
        Ok(Self { elements }.into())
    }

    fn create_from_elements(elements: Box<[Instruction]>) -> Instruction {
        let mut array = Vec::new();
        for instruction in &*elements {
            let Instruction::Variable(variable) = instruction else {
                return Self { elements }.into();
            };
            array.push(variable.clone());
        }
        Instruction::Variable(Variable::Tuple(array.into()))
    }
}

impl Exec for Tuple {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let elements = interpreter.exec(&self.elements)?;
        Ok(Variable::Tuple(elements))
    }
}

impl Recreate for Tuple {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Instruction {
        let elements = recreate_instructions(&self.elements, local_variables);
        Self::create_from_elements(elements)
    }
}

impl ReturnType for Tuple {
    fn return_type(&self) -> Type {
        let types = self.elements.iter().map(ReturnType::return_type).collect();
        Type::Tuple(types)
    }
}
