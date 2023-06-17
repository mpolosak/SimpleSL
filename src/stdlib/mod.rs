mod array;
mod iofunctions;
mod stdlib;
mod types;
use crate::intepreter::VariableMap;
use {
    array::add_array_functions, iofunctions::add_io_functions, stdlib::add_std_functions,
    types::add_types_functions,
};

pub fn add_std_lib(variables: &mut VariableMap) {
    add_std_functions(variables);
    add_array_functions(variables);
    add_io_functions(variables);
    add_types_functions(variables);
}
