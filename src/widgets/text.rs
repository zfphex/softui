use crate::*;
use fontdue::{Font, FontSettings};
use std::sync::atomic::{AtomicUsize, Ordering};
use taffy::{prelude::length, AvailableSpace, BoxSizing, Dimension, Size};

pub const FONT_DATA: &[u8] = include_bytes!("../../fonts/JetBrainsMono.ttf");

static mut DEFAULT_FONT_SIZE: AtomicUsize = AtomicUsize::new(18);
static mut DEFAULT_FONT: Option<Font> = None;

pub fn load_default_font() {
    let font = Font::from_bytes(FONT_DATA, FontSettings::default()).unwrap();
    set_default_font(font);
}

pub fn default_font() -> &'static Font {
    unsafe { DEFAULT_FONT.as_ref().unwrap() }
}

pub fn set_default_font(font: Font) {
    unsafe { DEFAULT_FONT = Some(font) };
}

pub fn set_default_font_size(size: usize) {
    unsafe { DEFAULT_FONT_SIZE.store(size, Ordering::Relaxed) }
}

pub fn default_font_size() -> usize {
    unsafe { DEFAULT_FONT_SIZE.load(Ordering::Relaxed) }
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub font_size: usize,
    pub layout: TaffyLayout,
    pub style: Style,
}

pub fn text<'a>(text: impl Into<Cow<'a, str>>) -> Text<'a> {
    Text {
        text: text.into(),
        font_size: default_font_size(),
        layout: TaffyLayout {
            box_sizing: BoxSizing::ContentBox,
            ..Default::default()
        },
        style: Style::new().fg(white()),
    }
}

impl<'a> Text<'a> {
    pub fn font_size(mut self, size: usize) -> Self {
        self.font_size = size;
        self
    }
}

impl<'a> Styling for Text<'a> {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<'a> Sizing for Text<'a> {
    fn layout_mut(&mut self) -> &mut TaffyLayout {
        &mut self.layout
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

    fn measure(&self, known: Size<Option<f32>>, available: Size<AvailableSpace>) -> Size<f32> {
        //TOOD: This is likely incorrect.
        let width = match available.width {
            AvailableSpace::Definite(px) => px as usize,
            AvailableSpace::MinContent => ctx_width(),
            AvailableSpace::MaxContent => ctx_width(),
        };

        let height = match available.height {
            AvailableSpace::Definite(px) => px as usize,
            AvailableSpace::MinContent => ctx_height(),
            AvailableSpace::MaxContent => ctx_height(),
        };

        let area = font::draw_text(
            &self.text,
            default_font(),
            0,
            0,
            self.font_size,
            0,
            1.0, //TODO: display scale
            Rect::new(0, 0, width, height),
            &mut [],
            white(),
            true,
        );

        Size {
            width: area.width as f32,
            height: area.height as f32,
        }
    }

    fn draw(&self, commands: &mut Vec<Command>, area: Rect) {
        if let Some(bg) = self.style.background_color {
            commands.push(Command {
                area,
                primative: Primative::Ellipse(0, None, bg),
            });
        }

        if let Some(fg) = self.style.foreground_color {
            let pad_left = self.layout.padding.left.into_raw().value();
            let pad_top = self.layout.padding.top.into_raw().value();
            let pad_right = self.layout.padding.right.into_raw().value();
            let pad_bottom = self.layout.padding.bottom.into_raw().value();

            commands.push(Command {
                area: Rect {
                    x: area.x + pad_left as usize,
                    y: area.y + pad_top as usize,
                    width: area.width + pad_right as usize,
                    height: area.height + pad_bottom as usize,
                },
                primative: Primative::Text(self.text.to_string(), self.font_size, fg),
            });
        }
    }
}
