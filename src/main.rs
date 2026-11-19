mod window;
use window::{Window};

use windows::{Win32::UI::WindowsAndMessaging::*};

fn main() {
    let mut window = Window::init_window();
    window.width = 800;
    window.height = 600;



    // Test functions
    println!("{}", window.width);
    println!("{}", window.get_height());
    let pos = window.get_position();
    println!("x: {}, y: {}", pos[0], pos[1]);
}
