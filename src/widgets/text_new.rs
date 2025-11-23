use taffy::{AvailableSpace, BoxSizing, Dimension, Size};

use crate::*;

pub fn text_new<'a>(text: impl Into<Cow<'a, str>>) -> TextNew<'a> {
    TextNew {
        text: text.into(),
        font_size: 22.0,
        style: Style::new(),
        layout: TaffyLayout {
            box_sizing: BoxSizing::ContentBox,
            size: Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            ..Default::default()
        },
        bg: Some(white()),
    }
}

#[derive(Debug)]
pub struct TextNew<'a> {
    pub text: Cow<'a, str>,
    pub font_size: f32,
    pub style: Style,
    pub layout: TaffyLayout,
    pub bg: Option<Color>,
}

impl<'a> TextNew<'a> {
    pub fn bg(mut self, bg: impl IntoColor) -> Self {
        self.bg = bg.into_color();
        self
    }
}

impl<'a> Widget<'a> for TextNew<'a> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        //HACK: Use the self defined self.bg instead.
        if let Some(style) = style {
            debug_assert!(style.background_color.is_none());
        }

        if let Some(bg) = self.bg {
            commands.push(Command {
                area,
                primative: Primative::Text(self.text.to_string(), self.font_size as usize, bg),
            });
        }
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }

fn measure(&self, _known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        let font = default_font().unwrap();

        // 1. Get correct vertical metrics from the font
        // 'new_line_size' typically includes ascent + descent + line_gap
        // If that feels too tall, use 'metrics.ascent - metrics.descent'
        let metrics = font.horizontal_line_metrics(self.font_size);
        // let line_height = metrics.new_line_size; 
        let line_height = 1.2;

        let max_width = match available_space.width {
            AvailableSpace::Definite(px) => px,
            AvailableSpace::MinContent => 0.0,
            AvailableSpace::MaxContent => f32::MAX,
        };

        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        let mut current_line_width: f32 = 0.0;

        for line in self.text.lines() {
            for ch in line.chars() {
                let metrics = font.metrics(ch, self.font_size);

                if current_line_width + metrics.advance_width > max_width {
                    width = width.max(current_line_width);
                    current_line_width = 0.0;
                    height += line_height;
                }

                current_line_width += metrics.advance_width;
            }

            width = width.max(current_line_width);
            current_line_width = 0.0;
            height += line_height;
        }

        if height == 0.0 && !self.text.is_empty() {
            height = line_height;
        }

        // 2. Round up to nearest pixel to prevent subpixel blur
        Size { 
            width: width.ceil(), 
            height: height.ceil() 
        }
    }
}
