use std::fmt;

#[derive(PartialEq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Self {
        Self {
            simple,
            combinators,
        }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self {
            simple: vec![],
            combinators: vec![],
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }

        write!(f, "{}", result)
    }
}

#[derive(PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

impl SimpleSelector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> Self {
        Self {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        Self {
            tag_name: None,
            id: None,
            classes: vec![],
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        if let Some(tag_name) = &self.tag_name {
            result.push_str(tag_name);
        }

        if let Some(id) = &self.id {
            result.push_str("#");
            result.push_str(id);
        }

        for class in self.classes.iter() {
            result.push_str(class)
        }

        write!(f, "{}", result)
    }
}
