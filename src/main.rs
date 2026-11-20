mod properties;

mod window;
use window::{Window};

mod div;
use div::{Div};
mod inithtml;
use inithtml::{HtmlBase};

fn main() {
    let mut window = Window::init_window_s();
    let mut div = Div::create_div();
    let htmlbase = HtmlBase::init_html();

    window.title("Testing name_subject_to_change");

    window.create_window();


    window.temp_run();
}
