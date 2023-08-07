use crate::{
    instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate},
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, GetType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct TypeFilter {
    array: Instruction,
    var_type: Type,
}

impl CreateInstruction for TypeFilter {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let array = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        let var_type = Type::from(inner.next().unwrap());
        if matches!(array.get_return_type(), Type::Array(_) | Type::EmptyArray) {
            Ok(Self { array, var_type }.into())
        } else {
            Err(Error::CannotDo2(array.get_return_type(), "?", var_type))
        }
    }
}

impl Exec for TypeFilter {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.array.exec(interpreter)?;
        match array {
            Variable::Array(array, _) => Ok(array
                .iter()
                .filter(|element| element.get_type().matches(&self.var_type))
                .cloned()
                .collect()),
            array => panic!("Tried to do {array} ? {}", self.var_type),
        }
    }
}

impl Recreate for TypeFilter {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let array = self.array.recreate(local_variables, interpreter)?;
        Ok(Self {
            array,
            var_type: self.var_type.clone(),
        }
        .into())
    }
}

impl GetReturnType for TypeFilter {
    fn get_return_type(&self) -> Type {
        Type::Array(self.var_type.clone().into())
    }
}

impl From<TypeFilter> for Instruction {
    fn from(value: TypeFilter) -> Self {
        Self::TypeFilter(value.into())
    }
}
