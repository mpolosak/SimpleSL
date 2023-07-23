use std::rc::Rc;

use super::{
    exec_instructions,
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    CreateInstruction, Instruction,
};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Array {
    instructions: Box<[Instruction]>,
    var_type: Type,
}

impl CreateInstruction for Array {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error> {
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| Instruction::new(arg, interpreter, local_variables))
            .collect::<Result<Box<_>, _>>()?;
        Ok(Self::create_from_instructions(instructions))
    }
}
impl Array {
    fn create_from_instructions(instructions: Box<[Instruction]>) -> Instruction {
        let mut iter = instructions.iter();
        if let Some(first) = iter.next() {
            let mut element_type = first.get_return_type();
            for instruction in iter {
                element_type = element_type.concat(instruction.get_return_type());
            }
            let var_type = Type::Array(element_type.into());
            let mut array = Vec::new();
            for instruction in instructions.iter() {
                let Instruction::Variable(variable) = instruction else {
                    return Self {
                        instructions,
                        var_type,
                    }
                    .into();
                };
                array.push(variable.clone());
            }
            Instruction::Variable(Variable::Array(array.into(), var_type))
        } else {
            Instruction::Variable(Variable::Array(Rc::new([]), Type::EmptyArray))
        }
    }
}

impl Exec for Array {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
    ) -> Result<crate::variable::Variable, Error> {
        let array = exec_instructions(&self.instructions, interpreter)?;
        Ok(Variable::Array(array, self.var_type.clone()))
    }
}

impl Recreate for Array {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let instructions = recreate_instructions(&self.instructions, local_variables, interpreter)?;
        Ok(Self::create_from_instructions(instructions))
    }
}

impl GetReturnType for Array {
    fn get_return_type(&self) -> Type {
        self.var_type.clone()
    }
}

impl From<Array> for Instruction {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}
