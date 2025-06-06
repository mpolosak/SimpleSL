mod convert;
mod fs;
mod io;
mod math;
pub(crate) mod operators;
mod string;
pub use self::{
    convert::convert_var, fs::fs_var, io::add_io_var, math::math_var, operators::OPERATORS,
    string::add_string, string::add_string_var,
};
use crate as simplesl;
use crate::variable::Variable;
use lazy_static::lazy_static;
use simplesl_macros::var;

lazy_static! {
    pub static ref stdlib: Variable = {
        let convert = convert_var.clone();
        let fs = fs_var.clone();
        let io = add_io_var.clone();
        let math = math_var.clone();
        let operators = OPERATORS.clone();
        let string = add_string_var.clone();
        var!(struct{
            convert = convert,
            fs = fs,
            io = io,
            math = math,
            operators = operators,
            string = string
        })
    };
}
