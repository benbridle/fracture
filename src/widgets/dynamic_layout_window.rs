use super::super::components::{DynamicRect, Screen, Widget};

pub struct DynamicLayoutWindow {
    sub_widgets: Vec<(DynamicRect, Box<dyn Widget>)>,
}

impl DynamicLayoutWindow {
    pub fn new() -> DynamicLayoutWindow {
        DynamicLayoutWindow {
            sub_widgets: Vec::new(),
        }
    }

    pub fn add_sub_widget(&mut self, dynamic_rect: DynamicRect, widget: Box<dyn Widget>) {
        self.sub_widgets.push((dynamic_rect, widget));
    }
}

impl Widget for DynamicLayoutWindow {
    fn render(&self, screen: &mut Screen) {
        for (dynamic_rect, sub_widget) in &self.sub_widgets {
            let static_rect = dynamic_rect.to_static_rect(screen.to_area());
            let mut sub_widget_screen = Screen::from_static_rect(&static_rect);
            sub_widget.render(&mut sub_widget_screen);
            screen.draw_screen(static_rect.origin, &sub_widget_screen);
        }
    }
}
