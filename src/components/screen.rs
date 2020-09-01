use super::Point;
use super::ScreenCell;
use std::fmt;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    content: Vec<Vec<ScreenCell>>,
}
