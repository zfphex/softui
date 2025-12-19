use crate::*;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{Options, Transform, Tree},
};
use taffy::prelude::length;

pub fn svg_path<P: AsRef<std::path::Path>>(path: P, scale: f32, invert: bool) -> Svg {
    svg(&std::fs::read(path).unwrap(), scale, invert)
}

pub fn svg(data: &[u8], scale: f32, invert: bool) -> Svg {
    let tree = Tree::from_data(data, &Options::default()).unwrap();
    let width = tree.size().width() * scale;
    let height = tree.size().height() * scale;

    let mut pixmap = Pixmap::new(width.ceil() as u32, height.ceil() as u32).unwrap();
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());

    Svg {
        layout: TaffyLayout {
            size: taffy::Size {
                width: length(width),
                height: length(height),
            },
            ..Default::default()
        },
        pixmap,
        width,
        height,
        invert,
    }
}

#[derive(Debug)]
pub struct Svg {
    pub pixmap: Pixmap,
    pub layout: TaffyLayout,
    pub width: f32,
    pub height: f32,
    pub invert: bool,
}

pub fn svg_ref<'a>(svg: &'a Svg) -> SvgRef<'a> {
    SvgRef {
        pixmap: &svg.pixmap,
        area: Rect::new(0, 0, svg.width.ceil() as usize, svg.height.ceil() as usize),
        layout: svg.layout.clone(),
        invert: svg.invert,
    }
}

#[derive(Debug)]
pub struct SvgRef<'a> {
    pub pixmap: &'a Pixmap,
    pub area: Rect,
    pub layout: TaffyLayout,
    pub invert: bool,
}

impl<'a> Widget<'a> for SvgRef<'a> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        //TODO: Just assume the svg exists for now.
        let pixmap = unsafe { std::mem::transmute::<&'a Pixmap, &'static Pixmap>(self.pixmap) };
        commands.push(Command {
            area,
            primative: Primative::SVGUnsafe(pixmap, self.invert),
        });
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}
