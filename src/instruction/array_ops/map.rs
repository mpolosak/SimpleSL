use crate::{
    function::Function,
    instruction::{
        macros::binOp,
        traits::{CanBeUsed, ExecResult, ExecStop},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{Array, ReturnType, Type, Variable},
};
use std::{iter::zip, rc::Rc};

binOp!(Map, "@", cfi);

impl CanBeUsed for Map {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let Type::Function(function_type) = rhs else {
            return false;
        };
        if lhs == &Type::EmptyArray {
            return function_type.params.len() == 1
                || (function_type.params.len() == 1
                    && Type::Int.matches(&function_type.params[0]));
        }
        if let Type::Array(element_type) = lhs {
            let params = &function_type.params;
            return (params.len() == 1 && element_type.matches(&params[0]))
                || (params.len() == 2
                    && Type::Int.matches(&params[0])
                    && element_type.matches(&params[1]));
        }
        let Type::Tuple(types) = lhs else {
            return false;
        };
        let mut params_iter = function_type.params.iter();
        if function_type.params.len() == types.len() {
            return zip(types.iter(), params_iter).all(|(var_type, param_type)| {
                matches!(var_type, Type::Array(var_type) if var_type.matches(param_type))
            });
        }
        if function_type.params.len() != types.len() + 1 {
            return false;
        }
        let index_type = params_iter.next().unwrap();
        Type::Int.matches(index_type)
            && zip(types.iter(), params_iter).all(|(var_type, param_type)| {
                matches!(var_type, Type::Array(var_type) if var_type.matches(param_type))
            })
    }
}

impl Map {
    fn zip_map(arrays: Rc<[Variable]>, function: Rc<Function>) -> ExecResult {
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
            let array = (0..len)
                .map(|i| {
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    function.exec(&args)
                })
                .collect::<Result<Variable, ExecError>>()?;
            return Ok(array);
        }
        let array = (0..len)
            .map(|i| {
                let mut args = vec![i.into()];
                args.extend(arrays.iter().map(|array| array[i].clone()));
                function.exec(&args)
            })
            .collect::<Result<Variable, ExecError>>()?;
        Ok(array)
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let Variable::Function(function) = function else {
            panic!("Tried to do {array} @ {function}")
        };
        if let Variable::Tuple(arrays) = array {
            return Self::zip_map(arrays, function);
        }
        let Variable::Array(array) = array else {
            panic!("Tried to do {array} @ {function}")
        };
        if function.params.len() == 1 {
            return array
                .iter()
                .cloned()
                .map(|var| function.exec(&[var]))
                .collect::<Result<Variable, ExecError>>()
                .map_err(ExecStop::from);
        }
        array
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, var)| function.exec(&[index.into(), var]))
            .collect::<Result<Variable, ExecError>>()
            .map_err(ExecStop::from)
    }
}

impl ReturnType for Map {
    fn return_type(&self) -> Type {
        let Type::Function(function_type) = self.rhs.return_type() else {
            unreachable!()
        };
        [function_type.return_type.clone()].into()
    }
}
