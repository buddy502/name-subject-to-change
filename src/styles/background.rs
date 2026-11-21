use windows::{
    Win32::{
        Foundation::{COLORREF, HWND, RECT},
        Graphics::Gdi::{
            BeginPaint, CreateSolidBrush, DeleteObject, EndPaint, FillRect, PAINTSTRUCT,
        },
        UI::WindowsAndMessaging::GetClientRect,
    },
};

use crate::properties::Background;
use crate::properties::Color;
use crate::window::Window;
use crate::HtmlBase;

impl Background {
    pub fn background_color(&mut self, window: &Window, color: Color) {
        self.background_color = color;
        window.redraw();
    }
}

pub unsafe fn on_paint(hwnd: HWND, htmlbase: Option<&HtmlBase>) {
    let mut ps = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

    if let Some(bg) = htmlbase.and_then(|html| html.styles.background.as_ref()) {
        let c = &bg.background_color;
        let colorref = COLORREF(c.r as u32 | (c.g as u32) << 8 | (c.b as u32) << 16);

        let mut rect = RECT::default();
        let _ = unsafe { GetClientRect(hwnd, &mut rect) };

        let brush = unsafe { CreateSolidBrush(colorref) };
        unsafe { FillRect(hdc, &rect, brush) };
        let _ = unsafe { DeleteObject(brush.into()) };
    }

    let _ = unsafe { EndPaint(hwnd, &ps) };
}
