use super::Instruction;
use crate::error::Error;
use crate::function::{Param, Params};
use crate::variable_type::GetType;
use std::iter::zip;

pub fn check_args(var_name: &str, params: &Params, args: &Vec<Instruction>) -> Result<(), Error> {
    match params.catch_rest {
        Some(_) if args.len() < params.standard.len() => {
            return Err(Error::WrongNumberOfArguments(
                String::from(var_name),
                params.standard.len(),
            ));
        }
        None if args.len() != params.standard.len() => {
            print!("E1");
            return Err(Error::WrongNumberOfArguments(
                String::from(var_name),
                params.standard.len(),
            ));
        }
        _ => (),
    }

    for (arg, Param { name, var_type }) in zip(args, &params.standard) {
        if !arg.get_type().matches(var_type) {
            print!("E3{}", arg.get_type());
            return Err(Error::WrongType(name.clone(), var_type.clone()));
        }
    }
    Ok(())
}
