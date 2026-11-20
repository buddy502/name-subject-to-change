mod properties;

mod window;
use window::{Window};

mod div;
use div::{Div};

fn main() {
    let mut window = Window::init_window_s();
    let mut div = Div::create_div();

    window.create_window();

    window.temp_run();

    window.destroy_window();
}
