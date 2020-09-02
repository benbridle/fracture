use fracture;
use fracture::components::DynamicRect;
use fracture::widgets::*;

fn main() {
    let mut dynamic_layout = DynamicLayoutWindow::new();
    dynamic_layout.add_sub_widget(DynamicRect::new(0, 0, 10, 5), Box::new(TestWidget::new()));
    dynamic_layout.add_sub_widget(
        DynamicRect::new(-20, -5, -10, -2),
        Box::new(TestWidget::new()),
    );
    let mut sub_window = DecoratedWindow::new(Some(String::from("Sub window")));
    let mut d_window = DecoratedWindow::new(Some(String::from("Root widget")));
    let test_widget = TestWidget::new();
    sub_window.set_sub_widget(Box::new(test_widget));
    dynamic_layout.add_sub_widget(DynamicRect::new(15, 3, -15, -3), Box::new(sub_window));
    d_window.set_sub_widget(Box::new(dynamic_layout));
    let fracture = fracture::Fracture::new(d_window);
    fracture.render_to_viewport();
}
