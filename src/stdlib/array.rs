use crate::{
    error::Error,
    function::{Function, NativeFunction, Param, Params},
    interpreter::Interpreter,
    variable::{Generics, Type, Variable},
};
use simplesl_macros::export_function;
use std::{iter::zip, rc::Rc};

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function(generics = "<T: any>", return_type = "[T]")]
    fn new_array(length: i64, #[var_type("T")] value: Variable) -> Result<Rc<[Variable]>, Error> {
        if length < 0 {
            return Err(Error::CannotBeNegative("length"));
        }
        let array = (0..length).map(|_| value.clone()).collect();
        Ok(array)
    }

    #[export_function(generics = "<T: any; S: any>", return_type = "[T|S]")]
    fn array_concat(
        #[var_type("[T]")] array1: &[Variable],
        #[var_type("[S]")] array2: &[Variable],
    ) -> Rc<[Variable]> {
        array1.iter().chain(array2).cloned().collect()
    }

    #[export_function(generics = "<T: any; S: any>", return_type = "[S]")]
    fn for_each(
        interpreter: &mut Interpreter,
        #[var_type("[T]")] array: &[Variable],
        #[var_type("function(T)->S")] function: Rc<dyn Function>,
    ) -> Result<Rc<[Variable]>, Error> {
        let new_array = array
            .iter()
            .map(|var| function.exec("function", interpreter, &[var.clone()]))
            .collect::<Result<Rc<[Variable]>, Error>>()?;
        Ok(new_array)
    }

    #[export_function(generics = "<T: any>", return_type = "[T]")]
    fn filter(
        interpreter: &mut Interpreter,
        #[var_type("[T]")] array: &[Variable],
        #[var_type("function(T)->int")] function: Rc<dyn Function>,
    ) -> Result<Rc<[Variable]>, Error> {
        let mut new_array = Vec::new();
        for element in array.iter() {
            if function.exec("function", interpreter, &[element.clone()])? != Variable::Int(0) {
                new_array.push(element.clone());
            }
        }
        Ok(new_array.into())
    }

    #[export_function(generics = "<T: any; S: any>", return_type = "[S]")]
    fn reduce(
        interpreter: &mut Interpreter,
        #[var_type("[T]")] array: &[Variable],
        #[var_type("S")] initial_value: Variable,
        #[var_type("function(S, T)->S")] function: Rc<dyn Function>,
    ) -> Result<Variable, Error> {
        array.iter().try_fold(initial_value, |acc, current| {
            function.exec("function", interpreter, &[acc, current.clone()])
        })
    }

    #[export_function(name = "zip", generics = "<T: any; S: any>", return_type = "[(T,S)]")]
    fn array_zip(
        #[var_type("[T]")] array1: &[Variable],
        #[var_type("[S]")] array2: &[Variable],
    ) -> Rc<[Variable]> {
        zip(array1.iter(), array2.iter())
            .map(|(element1, element2)| {
                Variable::Tuple(Rc::new([element1.clone(), element2.clone()]))
            })
            .collect()
    }

    #[export_function(generics = "<T: any>", return_type = "[T]")]
    fn recsub(
        interpreter: &mut Interpreter,
        #[var_type("[T]")] array: &[Variable],
        #[var_type("function([T])->T")] function: Rc<dyn Function>,
        n: i64,
    ) -> Result<Rc<[Variable]>, Error> {
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
