use crate::variable::Type;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TaggedType {
    pub tag: Arc<str>,
    pub payload: Option<Type>,
}

impl TaggedType {
    pub fn none() -> Self {
        Self::without_payload("None")
    }

    pub fn new(tag: impl Into<Arc<str>>, payload: Option<Type>) -> Self {
        Self {
            tag: tag.into(),
            payload,
        }
    }

    pub fn with_payload(tag: impl Into<Arc<str>>, payload: Type) -> Self {
        Self::new(tag, Some(payload))
    }

    pub fn without_payload(tag: impl Into<Arc<str>>) -> Self {
        Self::new(tag, None)
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.tag == other.tag
            && match (&self.payload, &other.payload) {
                (Some(left), Some(right)) => left.matches(right),
                (None, None) => true,
                _ => false,
            }
    }

    pub fn concat(self, other: Self) -> Option<Self> {
        if self.tag != other.tag {
            return None;
        }
        let payload = match (self.payload, other.payload) {
            (Some(left), Some(right)) => Some(left.concat(right)),
            (None, None) => None,
            _ => return None,
        };
        Some(Self::new(self.tag, payload))
    }

    pub fn conjoin(&self, other: &Self) -> Option<Self> {
        if self.tag != other.tag {
            return None;
        }
        let payload = match (&self.payload, &other.payload) {
            (Some(left), Some(right)) => {
                let payload = left.conjoin(right);
                if payload == Type::Never {
                    return None;
                }
                Some(payload)
            }
            (None, None) => None,
            _ => return None,
        };
        Some(Self::new(self.tag.clone(), payload))
    }
}

impl std::fmt::Display for TaggedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.tag)?;
        if let Some(payload) = &self.payload {
            write!(f, "({payload})")?;
        }
        Ok(())
    }
}
