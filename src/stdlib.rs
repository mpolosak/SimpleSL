mod convert;
mod fs;
mod io;
mod math;
pub(crate) mod operators;
mod string;
pub use self::{
    convert::add_convert_var, fs::add_fs_var, io::add_io_var, math::add_math_var,
    operators::OPERATORS, string::add_string, string::add_string_var,
};
use crate::interpreter::Interpreter;

/// Add all of standard library to Interpreter
pub fn add_all(interpreter: &mut Interpreter) {
    interpreter.insert("io".into(), add_io_var.clone());
    interpreter.insert("convert".into(), add_convert_var.clone());
    interpreter.insert("string".into(), add_string_var.clone());
    interpreter.insert("fs".into(), add_fs_var.clone());
    interpreter.insert("math".into(), add_math_var.clone());
    interpreter.insert("operators".into(), OPERATORS.clone());
}
