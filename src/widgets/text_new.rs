use taffy::{AvailableSpace, BoxSizing, Dimension, Size};

use crate::*;

pub fn text_new<'a>(text: impl Into<Cow<'a, str>>) -> TextNew<'a> {
    TextNew {
        text: text.into(),
        line_height: 1.2,
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
    pub line_height: f32,
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
        let canvas_width = ctx_width();
        let font = default_font();

        let mut y = 0.0;
        let x = 0.0;

        let mut max_x = 0.0;
        let mut max_y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        let line_height = self.line_height;

        'line: for line in self.text.lines() {
            let mut glyph_x = x;

            'char: for char in line.chars() {
                let (metrics, _) = font.rasterize(char, self.font_size);

                let glyph_y = y - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

                for y in 0..metrics.height {
                    for x in 0..metrics.width {
                        let offset = self.font_size + glyph_y + y as f32;

                        //We can't render off of the screen, mkay?
                        if offset < 0.0 {
                            continue;
                        }

                        if max_x < x as f32 + glyph_x {
                            max_x = x as f32 + glyph_x;
                        }

                        if max_y < offset {
                            max_y = offset;
                        }
                    }
                }

                glyph_x += metrics.advance_width;

                if glyph_x >= canvas_width as f32 {
                    break 'line;
                }
            }

            y += self.font_size + line_height;
        }

        height = (max_y + 1.0 - y);
        width = (max_x + 1.0 - x);

        Size {
            width: width,
            height: height,
        }
    }
}
