mod window;
use window::{Window};

mod div;

fn main() {
    let mut window = Window::init_window_s();

    window.create_window();

    window.temp_run();

    window.destroy_window();
}
