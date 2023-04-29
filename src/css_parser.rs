use crate::{css::{
    declaration::Declaration,
    rule::Rule,
    selector::{Selector, SimpleSelector}, value::Value,
}, utils::{translate_color, translate_length}};
use std::{iter::Peekable, str::Chars};

use super::css::stylesheet::Stylesheet;

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser<'a> {
    pub fn new(full_css: &'a str) -> Self {
        Self {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    pub fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();

            if selector != Selector::default() {
                selectors.push(selector);
            }

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        }

        self.chars.next();
        selectors
    }

    pub fn parse_selector(&mut self) -> Selector {
        let mut simple_selector = SimpleSelector::default();
        let mut selector = Selector::default();

        self.consume_while(char::is_whitespace);

        simple_selector.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_ident(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let mut multiple_ids = false;
        while self
            .chars
            .peek()
            .map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace())
        {
            match self.chars.peek() {
                Some(&c) if c == '#' => {
                    self.chars.next();
                    if simple_selector.id.is_some() || multiple_ids {
                        simple_selector.id = None;
                        multiple_ids = true;
                    } else {
                        simple_selector.id = self.parse_id();
                    }
                }
                Some(&c) if c == '.' => {
                    self.chars.next();
                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        simple_selector.classes.push(class_name);
                    }
                }
                _ => {
                    self.consume_while(|c| c != ',' && c != '{');
                }
            }
        }

        if simple_selector != SimpleSelector::default() {
            selector.simple.push(simple_selector);
        }

        selector
    }

    pub fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);

            let property = self.consume_while(|x| x != ':').to_lowercase();

            self.chars.next();

            self.consume_while(char::is_whitespace);

            let value = self.consume_while(|c| c != ';' && c != '\n' && c != '}').to_lowercase();

            let value_enum = match &property[..] {
                "background-color" | "border-color" | "color" => {
                    Value::Color(translate_color(&value))
                }
                "margin-right" |
                "margin-bottom" |
                "margin-left" |
                "margin-top" |
                "padding-right" |
                "padding-bottom" |
                "padding-left" |
                "padding-top" |
                "border-right-width" |
                "border-bottom-width" |
                "border-left-width" |
                "border-top-width" |
                "height" |
                "width" => translate_length(&value),
                _ => Value::Other(value),
            };

            let declaration = Declaration::new(property, value_enum);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                declarations.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);
                if self.chars.peek().map_or(false, |c| *c == '}') {
                    declarations.push(declaration);
                }
            }
            self.consume_while(char::is_whitespace);
        }

        self.chars.next();
        declarations
    }

    pub fn consume_while<F>(&mut self, condition: F) -> String
    where 
        F: Fn(char) -> bool, 
    {
        let mut result = String::new();

        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }

    pub fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();

        match self.chars.peek() {
            Some(&c) => if is_valid_start_ident(c) {
                identifier.push_str(&self.consume_while(is_valid_ident));
            },
            _ => {}
        }

        identifier.to_lowercase()
    }

    pub fn parse_id(&mut self) -> Option<String> {
        // let mut id = String::new();

        // match self.chars.peek() {
        //     Some(&c) => if is_valid_start_ident(c) {
        //         id.push_str(&self.consume_while(is_valid_ident));
        //     },
        //     _ => {}
        // }

        // Some(id)
        let identifier = &self.parse_identifier()[..];
        match identifier {
            "" => None,
            // @ is used to bind the variables to the pattern
            s @ _ => Some(s.to_string()),
        }
    }
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::{fs::File, io::Result};

    use super::*;

    fn get_file() -> Result<String> {
        let mut contents = String::new();
        let mut file = File::open("example.css")?;
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    #[test]
    fn test_parse_selectors() {
        let data = get_file().unwrap();

        println!("{data}");
    }
}
