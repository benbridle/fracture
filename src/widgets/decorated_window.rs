use super::super::components::{BorderChars, Point, Screen, Widget};

pub struct DecoratedWindow {
    title: Option<String>,
    sub_widget: Option<Box<dyn Widget>>,
}

impl DecoratedWindow {
    pub fn new(title: Option<String>) -> DecoratedWindow {
        DecoratedWindow {
            title,
            sub_widget: None,
        }
    }

    pub fn set_sub_widget(&mut self, widget: Box<dyn Widget>) {
        self.sub_widget = Some(widget);
    }

    pub fn render_sub_widget(&self, screen: &mut Screen) {
        if let Some(sub_widget) = &self.sub_widget {
            sub_widget.render(screen)
        }
    }
}

impl Widget for DecoratedWindow {
    fn render(&self, screen: &mut Screen) {
        let mut sub_widget_screen = Screen::new(screen.width - 2, screen.height - 2);
        self.render_sub_widget(&mut sub_widget_screen);
        screen.draw_screen(Point::new(1, 1), &sub_widget_screen);

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
