use super::{Array, Variable};
use crate::function::Function;
use std::{convert::Infallible, sync::Arc};

impl TryFrom<Variable> for bool {
    type Error = Variable;
    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_bool()
    }
}

impl TryFrom<&Variable> for bool {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_bool().copied().ok_or(())
    }
}

impl TryFrom<Variable> for i64 {
    type Error = Variable;
    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_int()
    }
}

impl TryFrom<&Variable> for i64 {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_int().copied().ok_or(())
    }
}

impl TryFrom<Variable> for f64 {
    type Error = Variable;
    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_float()
    }
}

impl TryFrom<&Variable> for f64 {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_float().copied().ok_or(())
    }
}

impl TryFrom<Variable> for Arc<str> {
    type Error = Variable;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_string()
    }
}

impl TryFrom<&Variable> for Arc<str> {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_string().cloned().ok_or(())
    }
}

impl<'a> TryFrom<&'a Variable> for &'a str {
    type Error = ();

    fn try_from(value: &'a Variable) -> Result<Self, Self::Error> {
        value.as_string().map(|val| val.as_ref()).ok_or(())
    }
}

impl TryFrom<Variable> for Arc<Array> {
    type Error = Variable;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_array()
    }
}

impl TryFrom<&Variable> for Arc<Array> {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_array().cloned().ok_or(())
    }
}

impl<'a> TryFrom<&'a Variable> for &'a Array {
    type Error = ();

    fn try_from(value: &'a Variable) -> Result<Self, Self::Error> {
        value.as_array().map(|val| val.as_ref()).ok_or(())
    }
}

impl<'a> TryFrom<&'a Variable> for &'a [Variable] {
    type Error = ();

    fn try_from(value: &'a Variable) -> Result<Self, Self::Error> {
        value.as_array().map(|val| val.as_ref().as_ref()).ok_or(())
    }
}

impl TryFrom<Variable> for Arc<Function> {
    type Error = Variable;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        value.into_function()
    }
}

impl TryFrom<&Variable> for Arc<Function> {
    type Error = ();

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        value.as_function().ok_or(()).cloned()
    }
}

impl<'a> TryFrom<&'a Variable> for &'a Arc<Function> {
    type Error = ();

    fn try_from(value: &'a Variable) -> Result<Self, Self::Error> {
        value.as_function().ok_or(())
    }
}

impl TryFrom<&Variable> for Variable {
    type Error = Infallible;

    fn try_from(value: &Variable) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}
