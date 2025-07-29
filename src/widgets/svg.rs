use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};

pub fn svg<P: AsRef<std::path::Path>>(path: P, width: usize, height: usize, scale: f32) -> Svg {
    Svg::new(path, width, height, scale)
}

#[derive(Debug)]
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

impl<'a> Widget<'a> for Svg {
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        //TODO: Just assume the svg exists for now.
        let pixmap = unsafe { extend_lifetime(&self.pixmap) };
        commands.push(Command {
            area: self.area,
            primative: Primative::SVGUnsafe(pixmap),
        });
    }
}
