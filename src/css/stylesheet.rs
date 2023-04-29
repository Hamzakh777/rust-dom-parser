
use std::default::Default;
use std::fmt::{self};

use super::rule::Rule;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self {
            rules
        }
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self {
            rules: vec![]
        }
    }
}

impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str(r"\n\s");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    }
}