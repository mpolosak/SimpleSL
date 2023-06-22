mod array;
mod iofunctions;
mod math;
mod stdlib;
mod string;
mod types;
use crate::interpreter::VariableMap;
use {
    array::add_array_functions, iofunctions::add_io_functions, math::add_math_functions,
    stdlib::add_std_functions, types::add_types_functions,
};

pub fn add_std_lib(variables: &mut VariableMap) {
    add_std_functions(variables);
    add_array_functions(variables);
    add_io_functions(variables);
    add_types_functions(variables);
    add_math_functions(variables);
    string::add_functions(variables);
}
