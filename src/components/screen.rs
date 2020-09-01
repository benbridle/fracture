use super::Point;
use super::ScreenCell;
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
}
