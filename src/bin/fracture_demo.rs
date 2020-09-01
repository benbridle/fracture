use fracture;
use fracture::components::Widget;

fn main() {
    println!("Fracture \x1b[1mTUI\x1b[0m library");
    println!("{:?}", fracture::components::Point::new(1, 2));
    let mut s = fracture::components::Screen::new(10, 5);
    let d_window = fracture::components::DecoratedWindow::new(Some(String::from("AAA")));
    d_window.render(&mut s);

    println!("{:?}", s);
    // let frac = fracture::Fracture::new();
    // frac.render();
}
