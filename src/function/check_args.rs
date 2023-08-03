use crate::{
    error::Error,
    function::{Param, Params},
    variable::Type,
};
use std::iter::zip;

pub fn check_args(var_name: &str, params: &Params, args: &[Type]) -> Result<(), Error> {
    if params.len() != args.len() {
        return Err(Error::WrongNumberOfArguments(var_name.into(), params.len()));
    }
    for (arg, Param { name, var_type }) in zip(args, params.iter()) {
        if !arg.matches(var_type) {
            return Err(Error::WrongType(name.clone(), var_type.clone()));
        }
    }
    Ok(())
}
