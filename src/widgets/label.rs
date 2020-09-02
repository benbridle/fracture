use super::super::components::{Point, Screen, Widget};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: String) -> Label {
        Label { text }
    }
}

impl Widget for Label {
    fn get_preferred_height(&self) -> Option<usize> {
        Some(1)
    }
    fn render(&self, screen: &mut Screen) {
        screen.draw_str(Point::zero(), self.text.as_str())
    }
}
