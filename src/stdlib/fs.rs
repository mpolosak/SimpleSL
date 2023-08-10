use crate::{
    function::{NativeFunction, Param, Params},
    interpreter::Interpreter,
    variable::{Type, Variable},
};
use simplesl_macros::export_function;
use std::{fs, io};

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn file_read_to_string(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }
    #[export_function]
    fn write_to_file(path: &str, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }
}
