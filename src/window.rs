/*
    Create a simple window in rust
    Base functionality, hud features not implemented
*/


use windows::{
    core::{
        w,
        HSTRING
    },
    Win32::{
        Graphics::Gdi:: {
            HBRUSH, COLOR_WINDOW, InvalidateRect
        },
        Foundation::{
            HWND,
            HINSTANCE,
            WPARAM, LPARAM, LRESULT
        },
        System::LibraryLoader::{GetModuleHandleW},
        UI::WindowsAndMessaging::{
            CreateWindowExW,
            DestroyWindow,
            RegisterClassW,
            GetMessageW, TranslateMessage, DispatchMessageW,
            WNDCLASSW, MSG, CS_HREDRAW, CS_VREDRAW,
            IsWindow,
            WINDOW_STYLE, WINDOW_EX_STYLE,
            DefWindowProcW, PostQuitMessage, WM_DESTROY,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE, CW_USEDEFAULT,
            GetWindowLongPtrW, GWLP_USERDATA, WM_PAINT,
        }
    }
};

use crate::HtmlBase;
use crate::styles::background::on_paint;

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *const HtmlBase;
            unsafe { on_paint(hwnd, ptr.as_ref()) };
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[derive(Default)]
pub struct Window {
    pub pwidth: i32,
    pub pheight: i32,
    pub position: [i32; 2],
    pub wtitle: HSTRING,

    pub window_flags: Option<WINDOW_STYLE>,
    pub hwnd: Option<HWND>,
}

//type Windows = Option<Box<Window>>;

impl Window {
    // Initialize a window struct with default values
    pub fn init_window_s() -> Self {
        Window {
            pwidth: 800,
            pheight: 600,
            wtitle: HSTRING::from(""),
            position: [CW_USEDEFAULT, CW_USEDEFAULT],
            window_flags: Some(WS_OVERLAPPEDWINDOW | WS_VISIBLE),
            ..Default::default()
        }
    }

    pub fn redraw(&self) {
        let _ = unsafe { InvalidateRect(self.hwnd, None, true) };
    }
    pub fn title(&mut self, title: &str) {
        self.wtitle = HSTRING::from(title);
    }
    pub fn get_title(&self) -> &HSTRING {
        &self.wtitle
    }

    // probably use Windows type for different function
    pub fn create_window(&mut self) {
        // !!! Were going to use self struct to build the window and 
        // then return that window

        let hmodule = unsafe { GetModuleHandleW(None) }.expect("Failed to get module handle");
        let hinstance: HINSTANCE = hmodule.into();

        // create register then create window
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance.into(),
            lpszClassName: w!("Window"),
            ..Default::default()
        };

        let class = unsafe {RegisterClassW(&wc)};
        assert!(class != 0, "RegisterClassW Failed: {:?}", windows::core::Error::from_win32());

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("Window"),
                self.get_title(),
                self.window_flags.unwrap_or(WS_OVERLAPPEDWINDOW | WS_VISIBLE),
                self.position[0], self.position[1],
                self.pwidth, self.pheight,
                None,
                None,
                Some(hinstance),
                None,
            )
        }
        .expect("Failed to create window");

        self.hwnd = Some(hwnd);

        // could initialize init_window or use parameters
    }

    pub fn temp_run(&self) {
        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg); // keyboard input to text
                DispatchMessageW(&msg); // required, calls wnd_proc
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if let Some(hwnd) = self.hwnd.take() {
            unsafe {
                if IsWindow(Some(hwnd)).as_bool() {
                    DestroyWindow(hwnd).expect("Failed to destroy window");
                }
            }
        }
    }
}
