use super::{Type, Typed, Variable};
use derive_more::Display;
use std::sync::RwLock;

#[derive(Display)]
#[display("{}", self.string(0))]
pub struct Mut {
    pub var_type: Type,
    pub variable: RwLock<Variable>,
}

impl Mut {
    pub(crate) fn string(&self, depth: u8) -> String {
        format!(
            "mut {} {}",
            self.var_type,
            self.variable.read().unwrap().debug(depth)
        )
    }
}

impl Typed for Mut {
    fn as_type(&self) -> Type {
        Type::Mut(self.var_type.clone().into())
    }
}
