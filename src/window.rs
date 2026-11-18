/*
    Create a simple window in rust
*/

enum WindowFlags {
    Windowed,
    Fullscreen,
    Borderless
}

#[derive(Default)]
pub struct Window {
    pub height: u16,
    pub width: u16,
    position: u16, // top left
    window_flags: Option<WindowFlags>,
}

// Lives on the heap, can fail
type Windows = Option<Box<Window>>;

impl Window {
    fn init_window() -> Self {
        Window { ..Default::default() }
    }
    fn create_window(&self, width: u16, height: u16, windowflags: Option<WindowFlags>) {
    }
}
