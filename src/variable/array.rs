use std::{
    fmt::{self, Display},
    ops::Deref,
    sync::Arc,
};

use crate::join_debug;

use super::{Type, Typed, Variable};

#[derive(PartialEq)]
pub struct Array {
    pub(crate) var_type: Type,
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
            var_type: array1.var_type.clone() | array2.var_type.clone(),
            elements: array1.iter().chain(array2.iter()).cloned().collect(),
        }
        .into()
    }

    pub fn new_repeat(value: Variable, len: usize) -> Self {
        let var_type = [value.as_type()].into();
        let elements = std::iter::repeat(value).take(len).collect();
        Self { var_type, elements }
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
        write!(f, "[{}]", join_debug(self.as_ref(), ", "))
    }
}

impl<T: Into<Arc<[Variable]>>> From<T> for Array {
    fn from(value: T) -> Self {
        let elements = value.into();
        let var_type = elements
            .iter()
            .map(Variable::as_type)
            .reduce(Type::concat)
            .map(|element_type| [element_type].into())
            .unwrap();
        Array { var_type, elements }
    }
}
