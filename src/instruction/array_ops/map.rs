use crate::{
    instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction, Recreate},
    interpreter::Interpreter,
    parse::Rule,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Map {
    array: Instruction,
    function: Instruction,
}

impl CreateInstruction for Map {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let array = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        let function = Instruction::new(inner.next().unwrap(), interpreter, local_variables)?;
        if Self::can_be_used(&array, &function) {
            Ok(Self { array, function }.into())
        } else {
            Err(Error::CannotDo(
                array.get_return_type(),
                "@",
                function.get_return_type(),
            ))
        }
    }
}

impl Map {
    fn can_be_used(instruction1: &Instruction, instruction2: &Instruction) -> bool {
        match (
            instruction1.get_return_type(),
            instruction2.get_return_type(),
        ) {
            (Type::Array(element_type), Type::Function(function_type)) => {
                let params = &function_type.params;
                params.len() == 1 && element_type.matches(&params[0])
            }
            (Type::EmptyArray, Type::Function(function_type)) => function_type.params.len() == 1,
            _ => false,
        }
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.array.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array, _), Variable::Function(function)) => array
                .iter()
                .map(|var| function.exec("function", interpreter, &[var.clone()]))
                .collect(),
            (array, function) => panic!("Tried to do {array} @ {function}"),
        }
    }
}

impl Recreate for Map {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let array = self.array.recreate(local_variables, interpreter)?;
        let function = self.function.recreate(local_variables, interpreter)?;
        Ok(Self { array, function }.into())
    }
}

impl GetReturnType for Map {
    fn get_return_type(&self) -> Type {
        let Type::Function(function_type) = self.function.get_return_type() else {panic!()};
        Type::Array(function_type.return_type.clone().into())
    }
}

impl From<Map> for Instruction {
    fn from(value: Map) -> Self {
        Self::Map(value.into())
    }
}
