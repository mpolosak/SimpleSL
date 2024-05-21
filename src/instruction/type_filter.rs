use crate::{
    instruction::{
        local_variable::LocalVariables, traits::ExecResult, Exec, Instruction, Recreate,
    },
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Typed},
    Error, ExecError,
};
use pest::iterators::Pair;

use super::InstructionWithStr;

#[derive(Debug)]
pub struct TypeFilter {
    array: InstructionWithStr,
    var_type: Type,
}

impl TypeFilter {
    pub fn create_instruction(
        array: InstructionWithStr,
        var_type: Pair<Rule>,
    ) -> Result<Instruction, Error> {
        let array_type = array.return_type();
        let var_type = Type::from(var_type);
        if !array_type.matches(&Type::Array(Type::Any.into())) {
            return Err(Error::CannotDo2(array_type, "?", var_type));
        }
        Ok(Self { array, var_type }.into())
    }
}

impl Exec for TypeFilter {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.array.exec(interpreter)?.into_array().unwrap();
        Ok(array
            .iter()
            .filter(|element| element.as_type().matches(&self.var_type))
            .cloned()
            .collect())
    }
}

impl Recreate for TypeFilter {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let array = self.array.recreate(local_variables)?;
        Ok(Self {
            array,
            var_type: self.var_type.clone(),
        }
        .into())
    }
}

impl ReturnType for TypeFilter {
    fn return_type(&self) -> Type {
        [self.var_type.clone()].into()
    }
}
