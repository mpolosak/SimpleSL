mod convert;
mod fs;
mod io;
mod math;
pub(crate) mod operators;
mod string;
pub use self::{
    convert::Convert, fs::FS, io::IO, math::Math, operators::Operators, string::String,
};
use crate as simplesl;
use crate::variable::Variable;
use simplesl_macros::{decls, export};

decls! {
    Std:=struct{
        convert=Convert,
        fs=FS,
        io=IO,
        math=Math,
        operators=Operators,
        string=String,
        len=Len
    }
}

#[export(Len)]
pub fn len(#[var_type([any]|string)] variable: &Variable) -> usize {
    match variable {
        Variable::Array(var) => var.len(),
        Variable::String(string) => string.chars().count(),
        _ => unreachable!(),
    }
}
