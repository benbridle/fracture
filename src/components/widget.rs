use super::Screen;

pub trait Widget {
    fn render(&self, screen: &mut Screen);
}
