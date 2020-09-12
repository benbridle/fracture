pub mod components;
pub mod widgets;
use components::*;
use termion;
use widgets::*;

pub struct Fracture {
    root_widget: DecoratedWindow,
}

impl Fracture {
    pub fn new(root_widget: DecoratedWindow) -> Fracture {
        Fracture { root_widget }
    }
    pub fn render_to_viewport(&self) {
        let size = termion::terminal_size().unwrap();
        let width = (size.0 - 20).into();
        let height = (size.1 - 5).into();

        let mut viewport_screen = Screen::new(width, height);
        self.root_widget.render(&mut viewport_screen);

        println!("{:?}", viewport_screen);
    }
}
