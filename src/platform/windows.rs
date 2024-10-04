use crate::{Backend, Window};

pub struct Windows {
    //window: window::Window,
    //framebuffer: Vec<u32>,
    //bitmap: BITMAPINFO,
    //dc: Option<*mut c_void>,
    //area: Rect,
}

impl Windows {
    pub fn new() -> Self {
        // TODO: Should the window struct in the `window` crate
        // have it's own framebuffer. Otherwise I'll need to create a wrapper.

        // create_window("Window", 600, 400)

        //Convert top, left, right, bottom to x, y, width, height.
        // let area = Rect::from(window.client_area());
        // let width = area.width;
        // let height = area.height;

        // let context = GetDC(hwnd);
        // let mut bitmap = BITMAPINFO::new(width, height);
        // let buffer_size = width as usize * height as usize;
        // let mut buffer = vec![0u32; buffer_size];

        Windows {}
    }
}

impl Backend for Windows {
    fn size(&self) -> crate::Rect {
        // self.area
        todo!()
    }

    fn buffer(&mut self) -> &mut [u32] {
        // self.framebuffer
        todo!()
    }

    fn resize(&self) {
        // let new_area = client_area(hwnd);
        // if new_area != area {
        //     area = new_area;
        //     width = area.width();
        //     height = area.height();

        //     buffer.clear();
        //     buffer.resize(width as usize * height as usize, 0);
        //     buffer.fill(fill_color);
        //     bitmap = BITMAPINFO::new(width, height);
        // }
    }

    fn present(&mut self) {
        // StretchDIBits(
        //     context,
        //     0,
        //     0,
        //     width,
        //     height,
        //     0,
        //     0,
        //     width,
        //     height,
        //     buffer.as_mut_ptr() as *const c_void,
        //     &bitmap,
        //     0,
        //     SRCCOPY,
        // );
    }

    fn event(&mut self) -> Option<crate::Event> {
        None
    }
}
