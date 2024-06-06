mod convert;
mod fs;
mod io;
mod math;
mod string;
use math::add_math;

pub use self::{convert::add_convert, fs::add_fs, io::add_io, string::add_string};
use crate::interpreter::Interpreter;

/// Add all of standard library to Interpreter
pub fn add_all(interpreter: &mut Interpreter) {
    add_io(interpreter);
    add_convert(interpreter);
    add_string(interpreter);
    add_fs(interpreter);
    add_math(interpreter)
}
