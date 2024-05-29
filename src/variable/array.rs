use std::{
    fmt::{self, Display},
    ops::Deref,
    sync::Arc,
};

use crate::join_debug;

use super::{Type, Typed, Variable};

#[derive(PartialEq)]
pub struct Array {
    pub(crate) element_type: Type,
    pub(crate) elements: Arc<[Variable]>,
}

impl Array {
    /// Returns `Rc<Array>` containing all elements of array1 and array2
    pub fn concat(array1: Arc<Self>, array2: Arc<Self>) -> Arc<Self> {
        if array1.is_empty() {
            return array2;
        }
        if array2.is_empty() {
            return array1;
        }
        Self {
            element_type: array1.element_type.clone() | array2.element_type.clone(),
            elements: array1.iter().chain(array2.iter()).cloned().collect(),
        }
        .into()
    }

    pub fn new_with_type(element_type: Type, elements: Arc<[Variable]>) -> Array {
        Self {
            element_type,
            elements,
        }
    }

    pub fn new_repeat(value: Variable, len: usize) -> Self {
        let element_type = value.as_type();
        let elements = std::iter::repeat(value).take(len).collect();
        Self {
            element_type,
            elements,
        }
    }

    pub fn element_type(&self) -> &Type {
        &self.element_type
    }
}

impl Typed for Array {
    fn as_type(&self) -> Type {
        [self.element_type.clone()].into()
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
        write!(f, "[{}]", join_debug(self.as_ref(), ", "))
    }
}

impl<T: Into<Arc<[Variable]>>> From<T> for Array {
    fn from(value: T) -> Self {
        let elements = value.into();
        let element_type = elements
            .iter()
            .map(Variable::as_type)
            .reduce(Type::concat)
            .unwrap();
        Array {
            element_type,
            elements,
        }
    }
}
