use super::{Array, Type, Variable};
use crate::{self as simplesl, ExecError};
use duplicate::duplicate_item;
use simplesl_macros::var_type;
use std::{io, sync::Arc};

pub trait TypeOf {
    fn type_of() -> Type;
}

impl TypeOf for () {
    fn type_of() -> Type {
        Type::Void
    }
}

#[duplicate_item(T; [bool]; [Result<bool, ExecError>])]
impl TypeOf for T {
    fn type_of() -> Type {
        Type::Bool
    }
}

#[duplicate_item(T; [i64]; [usize]; [u32]; [Result<i64, ExecError>];
    [Result<usize, ExecError>]; [Result<u32, ExecError>])]
impl TypeOf for T {
    fn type_of() -> Type {
        Type::Int
    }
}

#[duplicate_item(T; [f64]; [Result <f64, ExecError>])]
impl TypeOf for T {
    fn type_of() -> Type {
        Type::Float
    }
}

#[duplicate_item(T; [&str]; [Arc<str>]; [String]; [Result<Arc<&str>, ExecError>];
    [Result<Arc<str>, ExecError>]; [Result<String, ExecError>];)]
impl TypeOf for T {
    fn type_of() -> Type {
        Type::String
    }
}

#[duplicate_item(T; [Arc<Array>]; [Array]; [&Array]; [&[Variable]];
    [Result<Arc<Array>, ExecError>]; [Result<Array, ExecError>]; [Result<&Array, ExecError>];
    [Result<&[Variable], ExecError>]
)]
impl TypeOf for T {
    fn type_of() -> Type {
        var_type!([any])
    }
}

#[duplicate_item(T; [&Variable]; [Variable]; [Result<&Variable, ExecError>]; [Result<Variable, ExecError>])]
impl TypeOf for T {
    fn type_of() -> Type {
        var_type!(any)
    }
}

impl<T: TypeOf, S: TypeOf> TypeOf for Result<T, S> {
    fn type_of() -> Type {
        let ok = T::type_of();
        let err = S::type_of();
        var_type!(ok | err)
    }
}

impl TypeOf for io::Error {
    fn type_of() -> Type {
        var_type!(struct {
            error_code: int,
            msg: string
        })
    }
}

impl TypeOf for Option<bool> {
    fn type_of() -> Type {
        var_type!(bool | ())
    }
}

#[duplicate_item(T; [Option<i64>]; [Option<u32>]; [Option<i32>])]
impl TypeOf for T {
    fn type_of() -> Type {
        var_type!(int | ())
    }
}

impl TypeOf for Option<f64> {
    fn type_of() -> Type {
        var_type!(float | ())
    }
}

impl TypeOf for Option<String> {
    fn type_of() -> Type {
        var_type!(string | ())
    }
}
