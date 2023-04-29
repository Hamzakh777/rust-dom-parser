use std::{fmt, vec};

use super::{declaration::Declaration, selector::Selector};

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
        Self {
            selectors,
            declarations,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            selectors: vec![],
            declarations: vec![],
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut selector_result = String::new();
        let mut declaration_result = String::new();
        let tab = "    ";

        for selector in &self.selectors {
            if selector_result.len() > 0 {
                selector_result.push_str(", ");
            }
            selector_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            declaration_result.push_str(tab);
            declaration_result.push_str(&format!("{:?}", declaration));
            declaration_result.push_str("\n");
        }

        write!(f, "{} {{\n{}}}", selector_result, declaration_result)
    }
}
