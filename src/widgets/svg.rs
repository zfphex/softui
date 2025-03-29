use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};

pub struct Svg {
    pub pixmap: Pixmap,
    pub area: Rect,
}

impl Svg {
    pub fn new<P: AsRef<std::path::Path>>(path: P, width: usize, height: usize, scale: f32) -> Self {
        let tree = Tree::from_data(&std::fs::read(path).unwrap(), &Options::default()).unwrap();
        let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());

        Self {
            area: Rect::new(0, 0, pixmap.width() as usize, pixmap.height() as usize),
            pixmap,
        }
    }
}

impl Widget for Svg {
    type Layout = Self;

    fn primative(&self) -> Primative {
        let pixmap = unsafe { extend_lifetime(&self.pixmap) };
        Primative::SVGUnsafe(pixmap)
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn area_mut(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}
