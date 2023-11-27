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

impl Array {
    /// Returns Rc<Array> containing all elements of array1 and array2
    pub fn concat(array1: Rc<Self>, array2: Rc<Self>) -> Rc<Self> {
        if array1.is_empty() {
            array2
        } else if array2.is_empty() {
            array1
        } else {
            Self {
                var_type: array1.var_type.clone() | array2.var_type.clone(),
                elements: array1.iter().chain(array2.iter()).cloned().collect(),
            }
            .into()
        }
    }
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