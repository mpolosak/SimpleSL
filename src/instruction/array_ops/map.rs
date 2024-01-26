use crate::{
    binOp,
    function::Function,
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
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(Self { lhs, rhs }.into())
    }

    fn zip_map(
        interpreter: &mut Interpreter,
        arrays: Rc<[Variable]>,
        function: Rc<Function>,
    ) -> Result<Variable> {
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
        if function.params.len() == arrays.len() {
            return (0..len)
                .map(|i| {
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    function.exec(interpreter, &args)
                })
                .collect();
        }
        (0..len)
            .map(|i| {
                let mut args = vec![i.into()];
                args.extend(arrays.iter().map(|array| array[i].clone()));
                function.exec(interpreter, &args)
            })
            .collect()
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let Variable::Function(function) = function else {
            panic!("Tried to do {array} @ {function}")
        };
        if let Variable::Tuple(arrays) = array {
            return Self::zip_map(interpreter, arrays, function);
        }
        let Variable::Array(array) = array else {
            panic!("Tried to do {array} @ {function}")
        };
        if function.params.len() == 1 {
            return array
                .iter()
                .cloned()
                .map(|var| function.exec(interpreter, &[var]))
                .collect();
        }
        array
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, var)| function.exec(interpreter, &[index.into(), var]))
            .collect()
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
