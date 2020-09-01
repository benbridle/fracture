use fracture;
use fracture::components::Widget;

fn main() {
    let d_window = fracture::components::DecoratedWindow::new(Some(String::from("Root widget")));
    let fracture = fracture::Fracture::new(d_window);
    fracture.render_to_viewport();
}
