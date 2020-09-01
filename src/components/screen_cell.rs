use std::fmt;

#[derive(Copy, Clone)]
pub struct ScreenCell {
    pub character: char,
}

impl ScreenCell {
    pub fn new() -> ScreenCell {
        ScreenCell { character: ' ' }
    }
    pub fn from_char(c: char) -> ScreenCell {
        ScreenCell { character: c }
    }
}

impl fmt::Debug for ScreenCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.character)
    }
}
