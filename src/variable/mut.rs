use super::{Type, Typed, Variable};
use std::{fmt::Display, sync::RwLock};

pub struct Mut {
    pub var_type: Type,
    pub variable: RwLock<Variable>,
}

impl Mut {
    pub(crate) fn string(&self, depth: u8) -> String {
        format!(
            "mut {} {}",
            self.var_type,
            self.variable.read().unwrap().string(depth)
        )
    }
}

impl Display for Mut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string(0))
    }
}

impl Typed for Mut {
    fn as_type(&self) -> Type {
        Type::Mut(self.var_type.clone().into())
    }
}
