use std::fmt;

use super::value::Value;

pub struct Declaration {
    pub property: String,
    pub value: Value,
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Self {
        Self { property, value }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}
