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
