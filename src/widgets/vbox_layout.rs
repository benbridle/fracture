use super::super::components::{Point, Screen, Widget};

pub struct VBoxLayout {
    sub_widgets: Vec<Box<dyn Widget>>,
}

impl VBoxLayout {
    pub fn new() -> VBoxLayout {
        VBoxLayout {
            sub_widgets: Vec::new(),
        }
    }

    pub fn add_sub_widget(&mut self, widget: Box<dyn Widget>) {
        self.sub_widgets.push(widget);
    }
}

impl Widget for VBoxLayout {
    fn render(&self, screen: &mut Screen) {
        let mut remaining_height = screen.height;
        let mut ambivilent_count = 0;
        let mut heights = Vec::new();
        for sub_widget in &self.sub_widgets {
            heights.push(sub_widget.get_preferred_height());
            match sub_widget.get_preferred_height() {
                Some(height) => {
                    // Subtract height from remaining_height without overflowing
                    remaining_height = if remaining_height > height {
                        remaining_height - height
                    } else {
                        0
                    }
                }
                None => ambivilent_count = ambivilent_count + 1,
            }
        }
        let ambivilent_height = remaining_height / ambivilent_count;

        let mut y = 0;
        for sub_widget in &self.sub_widgets {
            let sub_widget_height = match sub_widget.get_preferred_height() {
                Some(height) => height,
                None => ambivilent_height,
            };
            let mut sub_widget_screen = Screen::new(screen.width, sub_widget_height);
            sub_widget.render(&mut sub_widget_screen);
            screen.draw_screen(Point::new(0, y), &sub_widget_screen);
            y = y + sub_widget_height;
        }
    }
}
