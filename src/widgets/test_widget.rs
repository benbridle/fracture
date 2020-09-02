// Basic pattern-drawing widget for debugging widget drawing
use super::super::components::{Point, Screen, Widget};

pub struct TestWidget {}

impl TestWidget {
    pub fn new() -> TestWidget {
        TestWidget {}
    }
}

impl Widget for TestWidget {
    fn render(&self, screen: &mut Screen) {
        // +1 is to fix gaps in center of widget
        for y in 0..screen.height / 2 + 1 {
            for x in 0..screen.width / 2 + 1 {
                // Convert the usize to a u8, shift into ascii alphabet range, cast to char
                let num = x + y;
                let character = if num < 10 {
                    (num as u8 + b'0') as char
                } else if num < 36 {
                    (num as u8 + b'A' - 10) as char
                } else if num < 62 {
                    (num as u8 + b'a' - 36) as char
                } else {
                    '+'
                };
                // Draw pattern from all four corners
                screen.draw_char(Point::new(x, y), character);
                screen.draw_char(Point::new(screen.width - 1 - x, y), character);
                screen.draw_char(Point::new(x, screen.height - 1 - y), character);
                screen.draw_char(
                    Point::new(screen.width - 1 - x, screen.height - 1 - y),
                    character,
                );
            }
        }
    }
}
