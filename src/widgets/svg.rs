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

pub fn svg_ref<'a>(svg: &'a Svg) -> SvgRef<'a> {
    SvgRef {
        pixmap: &svg.pixmap,
        area: svg.area,
    }
}

#[derive(Debug)]
pub struct SvgRef<'a> {
    pub pixmap: &'a Pixmap,
    pub area: Rect,
}

impl<'a> Widget<'a> for SvgRef<'a> {
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        //TODO: Just assume the svg exists for now.
        let pixmap = unsafe { std::mem::transmute::<&'a Pixmap, &'static Pixmap>(self.pixmap) };
        commands.push(Command {
            area: self.area,
            primative: Primative::SVGUnsafe(pixmap),
        });
    }
    
    fn desired_size(&self) -> (Unit, Unit) {
        todo!()
    }
}
