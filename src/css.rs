use std::fmt;

// This implementation only covers the basic css selectors
// We won't cover cascading in this implementation
pub mod declaration;
pub mod rule;
pub mod selector;
pub mod stylesheet;
pub mod value;

#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.b, self.b, self.a)
    }
}
