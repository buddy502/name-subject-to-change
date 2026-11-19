/*
    Create a simple window in rust
*/

enum WindowFlags {
    WINDOWED,
    FULLSCREEN,
    BORDERLESS 
}

#[derive(Default)]
pub struct Window {
    pub height: u16,
    pub width: u16,
    pub position: [u16; 2], // top left of window
    window_flags: Option<Vec<WindowFlags>>,
}

// Lives on the heap, can fail
type Windows = Option<Box<Window>>;

impl Window {
    pub fn init_window() -> Self {
        Window {..Default::default()}
    }
    // get private fields
    pub fn get_width(&self) -> u16 {self.width}
    pub fn get_height(&self) -> u16 {self.height}
    pub fn get_position(&self) -> [u16; 2] {[self.position[0], self.position[1]]}
   
    // probably use Windows type for different function
    pub fn create_window(&mut self, width: u16, height: u16, windowflags: Option<WindowFlags>) {
        // could initialize init_window or use parameters
        
    }
}
