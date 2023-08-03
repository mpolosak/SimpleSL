use crate::{
    function::{Function, NativeFunction, Param, Params},
    interpreter::Interpreter,
    variable::{Type, Variable},
    Error, Result,
};
use simplesl_macros::export_function;
use std::{iter::zip, rc::Rc};

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn new_array(length: i64, value: Variable) -> Result<Rc<[Variable]>> {
        if length < 0 {
            return Err(Error::CannotBeNegative("length"));
        }
        let array = (0..length).map(|_| value.clone()).collect();
        Ok(array)
    }

    #[export_function]
    fn for_each(
        interpreter: &mut Interpreter,
        array: &[Variable],
        #[var_type("function(any)->any")] function: Rc<dyn Function>,
    ) -> Result<Rc<[Variable]>> {
        let new_array = array
            .iter()
            .map(|var| function.exec("function", interpreter, &[var.clone()]))
            .collect::<Result<Rc<[Variable]>>>()?;
        Ok(new_array)
    }

    #[export_function]
    fn filter(
        interpreter: &mut Interpreter,
        array: &[Variable],
        #[var_type("function(any)->int")] function: Rc<dyn Function>,
    ) -> Result<Rc<[Variable]>> {
        let mut new_array = Vec::new();
        for element in array.iter() {
            if function.exec("function", interpreter, &[element.clone()])? != Variable::Int(0) {
                new_array.push(element.clone());
            }
        }
        Ok(new_array.into())
    }

    #[export_function]
    fn reduce(
        interpreter: &mut Interpreter,
        array: &[Variable],
        initial_value: Variable,
        #[var_type("function(any, any)->any")] function: Rc<dyn Function>,
    ) -> Result<Variable> {
        array.iter().try_fold(initial_value, |acc, current| {
            function.exec("function", interpreter, &[acc, current.clone()])
        })
    }

    #[export_function(name = "zip", return_type = "[[any]]")]
    fn array_zip(array1: &[Variable], array2: &[Variable]) -> Rc<[Variable]> {
        zip(array1.iter(), array2.iter())
            .map(|(element1, element2)| {
                Variable::Array(
                    Rc::new([element1.clone(), element2.clone()]),
                    Type::Array(Type::Any.into()),
                )
            })
            .collect()
    }

    #[export_function]
    fn recsub(
        interpreter: &mut Interpreter,
        array: &[Variable],
        #[var_type("function([any])->any")] function: Rc<dyn Function>,
        n: i64,
    ) -> Result<Rc<[Variable]>> {
        if n < 0 {
            return Err(Error::CannotBeNegative("n"));
        }
        let n = n as usize;
        if array.len() > n {
            let new_array = array.iter().take(n).cloned().collect();
            Ok(new_array)
        } else {
            let mut new_array: Vec<Variable> = array.into();
            for _ in 0..n - array.len() {
                new_array.push(function.exec(
                    "function",
                    interpreter,
                    &[Variable::Array(
                        new_array.clone().into(),
                        Type::Array(Type::Any.into()),
                    )],
                )?);
            }
            Ok(new_array.into())
        }
    }
}
