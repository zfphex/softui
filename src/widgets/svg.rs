use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use taffy::prelude::length;

pub fn svg<P: AsRef<std::path::Path>>(path: P, width: usize, height: usize, scale: f32) -> Svg {
    Svg::new(path, width, height, scale)
}

#[derive(Debug)]
pub struct Svg {
    pub pixmap: Pixmap,
    pub area: Rect,
    pub layout: TaffyLayout,
}

impl Svg {
    pub fn new<P: AsRef<std::path::Path>>(path: P, width: usize, height: usize, scale: f32) -> Self {
        let tree = Tree::from_data(&std::fs::read(path).unwrap(), &Options::default()).unwrap();
        let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());

        Self {
            area: Rect::new(0, 0, pixmap.width() as usize, pixmap.height() as usize),
            layout: TaffyLayout {
                size: taffy::Size {
                    width: length(width as f32),
                    height: length(height as f32),
                },
                ..Default::default()
            },
            pixmap,
        }
    }
}

pub fn svg_ref<'a>(svg: &'a Svg) -> SvgRef<'a> {
    SvgRef {
        pixmap: &svg.pixmap,
        area: svg.area,
        layout: svg.layout.clone(),
    }
}

#[derive(Debug)]
pub struct SvgRef<'a> {
    pub pixmap: &'a Pixmap,
    pub area: Rect,
    pub layout: TaffyLayout,
}

impl<'a> Widget<'a> for SvgRef<'a> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        //TODO: Just assume the svg exists for now.
        let pixmap = unsafe { std::mem::transmute::<&'a Pixmap, &'static Pixmap>(self.pixmap) };
        commands.push(Command {
            area,
            primative: Primative::SVGUnsafe(pixmap),
        });
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}
