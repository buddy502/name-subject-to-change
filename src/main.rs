use windows::{
    Win32::{
        UI::WindowsAndMessaging::{
            SetWindowLongPtrW, GWLP_USERDATA
        }
    }
};

mod properties;

mod window;
use window::{Window};

mod styles {
    pub mod background;
}

mod tags {
    pub mod div;
    pub mod html;
    pub mod body;
}
use tags::div::{Div};
use tags::html::{HtmlBase};

// !!! Bug: You shouldn't be able to change certain style elements on
// tags
// caveat for properties and tags is that everything is mutable
fn main() {
    let mut window = Window::init_window_s();

    let mut div = Div::create_div();
    let mut htmlbase = Box::new(HtmlBase::init_html());

    window.title("Testing name_subject_to_change");

    window.create_window();

    if let Some(hwnd) = window.hwnd {
        unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, &mut *htmlbase as *mut HtmlBase as isize); }
    }

    htmlbase.styles.background.as_mut().unwrap().background_color(&window, properties::Color::rgb(0, 120, 255));

    window.temp_run();
}
