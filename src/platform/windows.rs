use crate::{Backend, Rect};
use std::{ffi::c_void, pin::Pin};
use window::{client_area, create_window, GetDC, StretchDIBits, BITMAPINFO, RECT, SRCCOPY};

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Rect {
            x: 0,
            y: 0,
            width: rect.width(),
            height: rect.height(),
        }
    }
}

#[derive(Debug)]
pub struct Window {
    window: Pin<Box<window::Window>>,
    buffer: Vec<u32>,
    bitmap: BITMAPINFO,
    dc: Option<*mut c_void>,
    area: Rect,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        // TODO: Should the window struct in the `window` crate
        // have it's own framebuffer. Otherwise I'll need to create a wrapper.

        let window = create_window("Window", 600, 400);

        //Convert top, left, right, bottom to x, y, width, height.
        let area = Rect::from(window.client_area());
        let context = unsafe { GetDC(window.hwnd) };
        let mut bitmap = BITMAPINFO::new(area.width, area.height);
        let buffer_size = width as usize * height as usize;
        let mut buffer = vec![0u32; buffer_size];

        Self {
            window,
            buffer,
            bitmap,
            dc: Some(context),
            area,
        }
    }
}

impl Backend for Window {
    fn size(&self) -> crate::Rect {
        self.area
    }

    fn buffer(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    fn resize(&mut self) {
        let new_area = Rect::from(client_area(self.window.hwnd));
        if new_area != self.area {
            self.area = new_area;
            // self.area.width = self.area.width();
            // self.area.height = self.area.height();

            self.buffer.clear();
            self.buffer
                .resize(self.area.width as usize * self.area.height as usize, 0);
            // self.buffer.fill(fill_color);
            self.bitmap = BITMAPINFO::new(self.area.width, self.area.height);
        }
    }

    fn present(&mut self) {
        unsafe {
            StretchDIBits(
                self.dc.unwrap(),
                0,
                0,
                self.area.width,
                self.area.height,
                0,
                0,
                self.area.width,
                self.area.height,
                self.buffer.as_mut_ptr() as *const c_void,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }
    }

    //TODO: This is dumb, maybe make another crate that will share the same type, no reason to convert between this.
    //TODO: Exit is broken
    fn event(&mut self) -> Option<crate::Event> {
        match self.window.event() {
            Some(event) => match event {
                window::Event::Quit => Some(crate::Event::Quit),
                window::Event::Mouse(x, y) => Some(crate::Event::Mouse(x, y)),
                window::Event::Move => Some(crate::Event::Move),
                window::Event::Dpi(dpi) => Some(crate::Event::Dpi(dpi)),
                _ => None,
                // Event::Input(key, modifiers) => Some(crate::Event::Input(key, modifiers)),
            },
            None => None,
        }
    }
}
