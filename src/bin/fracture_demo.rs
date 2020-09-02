use fracture;
use fracture::widgets;

fn main() {
    let sub_widget = widgets::TestWidget::new();
    let sub_window = widgets::DecoratedWindow::new(Some(String::from("Sub window")));
    let mut d_window = widgets::DecoratedWindow::new(Some(String::from("Root widget")));
    d_window.set_sub_widget(Box::new(sub_widget));
    let fracture = fracture::Fracture::new(d_window);
    fracture.render_to_viewport();
}
