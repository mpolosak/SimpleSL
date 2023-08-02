use super::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate};
use crate::{
    error::Error,
    interpreter::Interpreter,
    parse::Rule,
    variable::{Generics, GetReturnType, Type, Variable},
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct SubstituteTypes {
    function: Instruction,
    generics: Generics,
}

impl CreateInstruction for SubstituteTypes {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error> {
        let mut inner = pair.into_inner();
        let function = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        if !matches!(function.get_return_type(), Type::Function(_)) {
            let var_type = Type::new_from_str(None, "function")?;
            return Err(Error::WrongType("instruction".into(), var_type));
        }
        let generics = Generics::new(None, inner.next().unwrap())?;
        Ok(Self { function, generics }.into())
    }
}

impl Exec for SubstituteTypes {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let Variable::Function(function) = self.function.exec(interpreter)? else {panic!()};
        Ok(function.simplify_generics(&self.generics)?.into())
    }
}

impl Recreate for SubstituteTypes {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error> {
        let function = self.function.recreate(local_variables, interpreter)?;
        Ok(Self {
            function,
            generics: self.generics.clone(),
        }
        .into())
    }
}

impl GetReturnType for SubstituteTypes {
    fn get_return_type(&self) -> Type {
        self.function.get_return_type()
    }
}

impl From<SubstituteTypes> for Instruction {
    fn from(value: SubstituteTypes) -> Self {
        Self::SubstituteTypes(value.into())
    }
}
