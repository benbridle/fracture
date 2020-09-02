use super::super::components::{Point, Screen, Widget};

pub struct FancyLabel {
    text: String,
}

impl FancyLabel {
    pub fn new(text: String) -> FancyLabel {
        FancyLabel { text }
    }
}

impl Widget for FancyLabel {
    fn get_preferred_height(&self) -> Option<usize> {
        Some(3)
    }
    fn render(&self, screen: &mut Screen) {
        for x in 0..self.text.chars().count() {
            screen.draw_str(Point::new(x, 0), "=");
            screen.draw_str(Point::new(x, 2), "=");
        }
        screen.draw_str(Point::new(0, 1), self.text.as_str());
    }
}
