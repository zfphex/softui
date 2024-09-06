use crate::Backend;

pub struct MacOS {}

impl Backend for MacOS {
    fn area(&self) -> crate::Rect {
        todo!()
    }

    fn buffer<'a>(&self) -> &'a mut [u32] {
        todo!()
    }

    fn resize(&self) {
        todo!()
    }

    fn present(&self) {
        todo!()
    }

    fn event(&self) -> Option<crate::Event> {
        todo!()
    }
}