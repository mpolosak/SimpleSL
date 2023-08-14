use crate::{
    function::{self, Function},
    interpreter::Interpreter,
    variable::{Type, Variable},
    Result,
};
use simplesl_macros::export_function;
use std::rc::Rc;

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn reduce(
        array: &[Variable],
        initial_value: Variable,
        #[var_type("function(any, any)->any")] function: Rc<Function>,
    ) -> Result<Variable> {
        array.iter().try_fold(initial_value, |acc, current| {
            function.exec(&[acc, current.clone()])
        })
    }
}
