mod array;
mod iofunctions;
mod math;
mod stdlib;
mod string;
mod types;
use crate::interpreter::VariableMap;

pub fn add_std_lib(variables: &mut VariableMap) {
    stdlib::add_functions(variables);
    array::add_functions(variables);
    iofunctions::add_functions(variables);
    types::add_functions(variables);
    math::add_functions(variables);
    string::add_functions(variables);
}
