use super::{
    exec_instructions,
    local_variable::LocalVariableMap,
    recreate_instructions,
    traits::{Exec, Recreate},
    Instruction,
};
use crate::{
    error::Error,
    interpreter::VariableMap,
    parse::Rule,
    variable::Variable,
    variable_type::{GetReturnType, Type},
};
use pest::iterators::Pair;

#[derive(Clone)]
pub struct Array {
    instructions: Vec<Instruction>,
    var_type: Type,
}

impl Array {
    pub fn new(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Self, Error> {
        let inner = pair.into_inner();
        let instructions = inner
            .map(|arg| Instruction::new(arg, variables, local_variables))
            .collect::<Result<Vec<_>, _>>()?;
        let mut iter = instructions.iter();
        let var_type = if let Some(first) = iter.next() {
            let mut element_type = first.get_return_type();
            for instruction in iter {
                element_type = element_type.concat(instruction.get_return_type());
            }
            Type::Array(element_type.into())
        } else {
            Type::EmptyArray
        };
        Ok(Self {
            instructions,
            var_type,
        })
    }
}

impl Exec for Array {
    fn exec(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<crate::variable::Variable, Error> {
        let array = exec_instructions(&self.instructions, interpreter, local_variables)?;
        Ok(Variable::Array(array.into(), self.var_type.clone()))
    }
}

impl Recreate for Array {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction {
        let instructions = recreate_instructions(self.instructions, local_variables, args);
        Self {
            instructions,
            var_type: self.var_type,
        }
        .into()
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
