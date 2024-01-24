use crate::{
    binOp,
    instruction::{traits::CanBeUsed, Exec, Instruction},
    interpreter::Interpreter,
    variable::{Array, ReturnType, Type, Variable},
    Result,
};
use std::{iter::zip, rc::Rc};

binOp!(Map, "@");

impl CanBeUsed for Map {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
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

impl Map {
    fn create_from_instructions(array: Instruction, function: Instruction) -> Result<Instruction> {
        Ok(Self::construct(array, function).into())
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        match (array, function) {
            (Variable::Array(array), Variable::Function(function))
                if function.params.len() == 1 =>
            {
                array
                    .iter()
                    .cloned()
                    .map(|var| function.exec(interpreter, &[var]))
                    .collect()
            }
            (Variable::Array(array), Variable::Function(function)) => array
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, var)| function.exec(interpreter, &[index.into(), var]))
                .collect(),
            (Variable::Tuple(arrays), Variable::Function(function))
                if function.params.len() == arrays.len() =>
            {
                let arrays: Box<[&Rc<Array>]> = arrays
                    .iter()
                    .map(|array| {
                        let Variable::Array(array) = array else {
                            unreachable!()
                        };
                        array
                    })
                    .collect();
                let len = arrays.iter().map(|array| array.len()).min().unwrap();
                (0..len)
                    .map(|i| {
                        let args: Box<[Variable]> =
                            arrays.iter().map(|array| array[i].clone()).collect();
                        function.exec(interpreter, &args)
                    })
                    .collect()
            }
            (Variable::Tuple(arrays), Variable::Function(function))
                if function.params.len() == arrays.len() + 1 =>
            {
                let arrays: Box<[&Rc<Array>]> = arrays
                    .iter()
                    .map(|array| {
                        let Variable::Array(array) = array else {
                            unreachable!()
                        };
                        array
                    })
                    .collect();
                let len = arrays.iter().map(|array| array.len()).min().unwrap();
                (0..len)
                    .map(|i| {
                        let mut args = vec![i.into()];
                        args.extend(arrays.iter().map(|array| array[i].clone()));
                        function.exec(interpreter, &args)
                    })
                    .collect()
            }
            (array, function) => panic!("Tried to do {array} {} {function}", Self::SYMBOL),
        }
    }
}

impl ReturnType for Map {
    fn return_type(&self) -> Type {
        let Type::Function(function_type) = self.lhs.return_type() else {
            unreachable!()
        };
        [function_type.return_type.clone()].into()
    }
}
