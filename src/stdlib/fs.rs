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

    #[export_function]
    fn copy_file(from: &str, to: &str) -> io::Result<()> {
        fs::copy(from, to)?;
        Ok(())
    }

    #[export_function]
    fn remove_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)
    }

    #[export_function]
    fn remove_dir(path: &str) -> io::Result<()> {
        fs::remove_dir(path)
    }

    #[export_function]
    fn remove_dir_all(path: &str) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    #[export_function]
    fn create_dir(path: &str) -> io::Result<()> {
        fs::create_dir(path)
    }

    #[export_function]
    fn create_dir_all(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    #[export_function]
    fn rename(from: &str, to: &str) -> io::Result<()> {
        fs::rename(from, to)
    }
}
