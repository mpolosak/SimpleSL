use super::{Type, Typed, Variable};
use crate as simplesl;
use simplesl_macros::var_type;
use std::{
    fmt::{self, Display},
    ops::Deref,
    sync::Arc,
};

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

    pub(crate) fn string(&self, depth: u8) -> String {
        format!(
            "[{}]",
            self.elements
                .iter()
                .map(|v| v.debug(depth + 1))
                .collect::<Box<[_]>>()
                .join(", ")
        )
    }
}

impl Typed for Array {
    fn as_type(&self) -> Type {
        let element_type = self.element_type.clone();
        var_type!([element_type])
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
        write!(f, "{}", self.string(0))
    }
}

impl<T: Into<Arc<[Variable]>>> From<T> for Array {
    fn from(value: T) -> Self {
        let elements = value.into();
        let element_type = elements
            .iter()
            .map(Variable::as_type)
            .reduce(Type::concat)
            .unwrap_or(Type::Never);
        Array {
            element_type,
            elements,
        }
    }
}
