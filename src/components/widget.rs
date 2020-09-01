use super::Point;
use super::Screen;

pub trait Widget {
    fn get_preferred_width(&self) -> usize;
    fn get_preferred_height(&self) -> usize;
    fn render(&self, screen: &mut Screen);
}

pub struct DecoratedWindow {}

impl DecoratedWindow {
    pub fn new() -> DecoratedWindow {
        DecoratedWindow {}
    }
}

impl Widget for DecoratedWindow {
    fn get_preferred_width(&self) -> usize {
        10
    }
    fn get_preferred_height(&self) -> usize {
        5
    }
    fn render(&self, screen: &mut Screen) {
        screen.draw_char(Point::zero(), 'T');
    }
}
