mod convert;
mod fs;
mod io;
mod math;
pub(crate) mod operators;
mod string;
pub use self::{
    convert::Convert, fs::FS, io::IO, math::Math, operators::OPERATORS, string::String,
};
use crate as simplesl;
use crate::variable::Variable;
use lazy_static::lazy_static;
use simplesl_macros::{export, var};

lazy_static! {
    pub static ref stdlib: Variable = {
        let operators = OPERATORS.clone();
        var!(struct{
            convert=Convert,
            fs=FS,
            io=IO,
            math=Math,
            operators,
            string=String,
            len=Len
        })
    };
}

#[export(Len)]
pub fn len(#[var_type([any]|string)] variable: &Variable) -> usize {
    match variable {
        Variable::Array(var) => var.len(),
        Variable::String(string) => string.chars().count(),
        _ => unreachable!(),
    }
}
