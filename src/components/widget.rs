use super::Point;
use super::Screen;

struct BorderChars {
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bot_left: char,
    bot_right: char,
}

pub trait Widget {
    fn get_preferred_width(&self) -> usize;
    fn get_preferred_height(&self) -> usize;
    fn render(&self, screen: &mut Screen);
}

pub struct DecoratedWindow {
    title: Option<String>,
}

impl DecoratedWindow {
    pub fn new(title: Option<String>) -> DecoratedWindow {
        DecoratedWindow { title }
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
        let right = screen.width - 1;
        let bottom = screen.height - 1;
        let border = BorderChars {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bot_left: '└',
            bot_right: '┘',
        };

        for x in 0..screen.width {
            screen.draw_char(Point::new(x, 0), border.horizontal);
            screen.draw_char(Point::new(x, bottom), border.horizontal);
        }
        for y in 0..screen.height {
            screen.draw_char(Point::new(0, y), border.vertical);
            screen.draw_char(Point::new(right, y), border.vertical);
        }
        screen.draw_char(Point::new(0, 0), border.top_left);
        screen.draw_char(Point::new(right, 0), border.top_right);
        screen.draw_char(Point::new(0, bottom), border.bot_left);
        screen.draw_char(Point::new(right, bottom), border.bot_right);

        if let Some(title) = &self.title {
            let padded_title = format!(" {} ", title);
            let x = (screen.width - padded_title.chars().count()) / 2;
            screen.draw_str(Point::new(x, 0), padded_title.as_str());
        }
    }
}
