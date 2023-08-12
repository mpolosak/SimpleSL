use std::{iter::zip, rc::Rc};

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
            Err(Error::CannotDo2(
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
                (params.len() == 1 && element_type.matches(&params[0]))
                    || (params.len() == 2
                        && Type::Int.matches(&params[0])
                        && element_type.matches(&params[1]))
            }
            (Type::EmptyArray, Type::Function(function_type)) => {
                function_type.params.len() == 1
                    || (function_type.params.len() == 1
                        && Type::Int.matches(&function_type.params[0]))
            }
            (Type::Tuple(types), Type::Function(function_type))
                if function_type.params.len() == types.len() =>
            {
                zip(types.iter(), function_type.params.iter()).all(|(var_type, param_type)| {
                    if let Type::Array(var_type) = var_type {
                        var_type.matches(param_type)
                    } else {
                        false
                    }
                })
            }
            (Type::Tuple(types), Type::Function(function_type))
                if function_type.params.len() == types.len() + 1 =>
            {
                let mut params_iter = function_type.params.iter();
                let index_type = params_iter.next().unwrap();
                Type::Int.matches(index_type)
                    && zip(types.iter(), params_iter).all(|(var_type, param_type)| {
                        if let Type::Array(var_type) = var_type {
                            var_type.matches(param_type)
                        } else {
                            false
                        }
                    })
            }
            _ => false,
        }
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.array.exec(interpreter)?;
        let function = self.function.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array, _), Variable::Function(function))
                if function.params.len() == 1 =>
            {
                array
                    .iter()
                    .cloned()
                    .map(|var| function.exec(interpreter, &[var]))
                    .collect()
            }
            (Variable::Array(array, _), Variable::Function(function)) => array
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, var)| function.exec(interpreter, &[index.into(), var]))
                .collect(),
            (Variable::Tuple(arrays), Variable::Function(function))
                if function.params.len() == arrays.len() =>
            {
                let arrays: Box<[&Rc<[Variable]>]> = arrays
                    .iter()
                    .map(|array| {
                        if let Variable::Array(array, _) = array {
                            array
                        } else {
                            panic!()
                        }
                    })
                    .collect();
                let len = arrays.iter().map(|array| array.len()).min().unwrap();
                let mut result = Vec::new();
                for i in 0..len {
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    result.push(function.exec(interpreter, &args)?);
                }
                Ok(result.into())
            }
            (Variable::Tuple(arrays), Variable::Function(function))
                if function.params.len() == arrays.len() + 1 =>
            {
                let arrays: Box<[&Rc<[Variable]>]> = arrays
                    .iter()
                    .map(|array| {
                        if let Variable::Array(array, _) = array {
                            array
                        } else {
                            panic!()
                        }
                    })
                    .collect();
                let len = arrays.iter().map(|array| array.len()).min().unwrap();
                let mut result = Vec::new();
                for i in 0..len {
                    let mut args = vec![i.into()];
                    args.extend(arrays.iter().map(|array| array[i].clone()));
                    result.push(function.exec(interpreter, &args)?);
                }
                Ok(result.into())
            }
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
