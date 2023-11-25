use std::{
    fmt::{self, Display},
    ops::Deref,
    rc::Rc,
};

use crate::join_debug;

use super::{Type, Typed, Variable};

#[derive(PartialEq)]
pub struct Array {
    pub var_type: Type,
    pub elements: Rc<[Variable]>,
}

impl Typed for Array {
    fn as_type(&self) -> Type {
        self.var_type.clone()
    }
}

impl Deref for Array {
    type Target = [Variable];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", join_debug(self, ", "))
    }
}
