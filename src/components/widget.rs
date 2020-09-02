use super::Screen;

pub trait Widget {
    fn render(&self, screen: &mut Screen);
    fn get_preferred_width(&self) -> Option<usize> {
        return None;
    }
    fn get_preferred_height(&self) -> Option<usize> {
        return None;
    }
}
