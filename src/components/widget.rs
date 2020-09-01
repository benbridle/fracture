use super::Screen;

pub trait Widget {
    fn get_preferred_width(&self) -> usize;
    fn get_preferred_height(&self) -> usize;
    fn render(&self, screen: &mut Screen);
}
