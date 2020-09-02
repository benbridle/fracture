use super::{Area, Point, ScreenCell, StaticRect};
use std::fmt;

pub struct Screen {
    pub width: usize,
    pub height: usize,
    content: Vec<Vec<ScreenCell>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            content: vec![vec![ScreenCell::new(); width]; height],
        }
    }
    pub fn from_point(point: &Point) -> Screen {
        Screen::new(point.x, point.y)
    }
    pub fn from_static_rect(rect: &StaticRect) -> Screen {
        Screen::new(rect.width, rect.height)
    }

    pub fn get_content(&self) -> &Vec<Vec<ScreenCell>> {
        &self.content
    }
    pub fn draw_char(&mut self, p: Point, c: char) {
        // No need to check if coords are >= 0, because they're unsigned
        if p.x < self.width && p.y < self.height {
            self.content[p.y][p.x].character = c;
        }
    }
    pub fn draw_str(&mut self, p: Point, s: &str) {
        for (x, character) in s.chars().enumerate() {
            self.draw_char(Point::new(x + p.x, p.y), character)
        }
    }
    pub fn draw_screen(&mut self, p: Point, s: &Screen) {
        for (y, row) in s.content.iter().enumerate() {
            for (x, screen_cell) in row.iter().enumerate() {
                self.draw_char(Point::new(x + p.x, y + p.y), screen_cell.character)
            }
        }
    }
    pub fn to_area(&self) -> Area {
        Area::new(self.width, self.height)
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("\n");
        for row in self.content.iter() {
            output.push_str("[");
            for screen_cell in row.iter() {
                output.push(screen_cell.character);
            }
            output.push_str("]\n");
        }
        write!(f, "{}", output)
    }
}
