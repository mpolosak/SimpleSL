mod convert;
mod fs;
mod io;
mod math;
pub(crate) mod operators;
mod string;
pub use self::{
    convert::add_convert, fs::add_fs, io::add_io, math::add_math, operators::add_operators,
    string::add_string,
};
use crate::interpreter::Interpreter;

/// Add all of standard library to Interpreter
pub fn add_all(interpreter: &mut Interpreter) {
    add_io(interpreter);
    add_convert(interpreter);
    add_string(interpreter);
    add_fs(interpreter);
    add_math(interpreter);
    add_operators(interpreter);
}
