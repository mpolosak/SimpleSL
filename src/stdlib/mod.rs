mod array;
mod iofunctions;
mod stdlib;
use crate::intepreter::VariableMap;
use {stdlib::add_std_functions,array::add_array_functions,
        iofunctions::add_io_functions};

pub fn add_std_lib(variables:&mut VariableMap){
    add_std_functions(variables);
    add_array_functions(variables);
    add_io_functions(variables);
}