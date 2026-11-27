use std::{fs, io};

#[derive(Debug, PartialEq)]
pub struct Source {
    name: Option<String>,
    content: String,
}

impl Source {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            content: content.into(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn load(path: impl Into<String>) -> io::Result<Self> {
        let name = path.into();
        let content = fs::read_to_string(&name)?;
        let name = Some(name);
        Ok(Self { name, content })
    }
}

impl<T: Into<String>> From<T> for Source {
    fn from(value: T) -> Self {
        Self {
            name: None,
            content: value.into(),
        }
    }
}
