use fracture;
use fracture::widgets;

fn main() {
    let d_window = widgets::DecoratedWindow::new(Some(String::from("Root widget")));
    let fracture = fracture::Fracture::new(d_window);
    fracture.render_to_viewport();
}
