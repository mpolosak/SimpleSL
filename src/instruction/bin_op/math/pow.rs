use crate::variable::{Array, Variable};
use crate::ExecError;
use std::sync::Arc;

pub fn exec(base: Variable, exp: Variable) -> Result<Variable, ExecError> {
    match (base, exp) {
        (_, Variable::Int(exp)) if exp < 0 => Err(ExecError::NegativeExponent),
        (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
        (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
        (base, Variable::Array(array)) => {
            let elements = array
                .iter()
                .cloned()
                .map(|exp| exec(base.clone(), exp))
                .collect::<Result<Arc<_>, _>>()?;
            let element_type = array.element_type().clone();
            Ok(Array {
                element_type,
                elements,
            }
            .into())
        }
        (Variable::Array(array), exp) => {
            let elements = array
                .iter()
                .cloned()
                .map(|base| exec(base, exp.clone()))
                .collect::<Result<Arc<_>, _>>()?;
            let element_type = array.element_type().clone();
            Ok(Array {
                element_type,
                elements,
            }
            .into())
        }
        (base, exp) => panic!("Tried to calc {base} * {exp}"),
    }
}
