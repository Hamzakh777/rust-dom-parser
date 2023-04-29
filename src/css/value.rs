use std::fmt;

use super::{Color, Unit};

pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Color(c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(s) => write!(f, "{:?}", s)
        }
    }
}
