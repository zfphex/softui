//! Font related logic
//!
use crate::*;

pub fn calculate_font_area() {}

pub fn draw_text(
    text: &str,
    font: &fontdue::Font,
    x: usize,
    y: usize,
    font_size: usize,
    //Zero is fine
    line_height: usize,
    display_scale: f32,
    window: Rect,
    buffer: &mut [u32],
    color: Color,
    skip_draw: bool,
) -> Rect {
    if text.is_empty() || font_size == 0 {
        return Rect::new(0, 0, 0, 0);
    }

    let x = scale(x, display_scale);
    let y = scale(y, display_scale);
    let font_size = scale(font_size, display_scale);
    let line_height = scale(line_height, display_scale);

    let mut area = Rect::new(x, y, 0, 0);
    let mut y = area.y;
    let x = area.x;

    let mut max_x = 0;
    let mut max_y = 0;

    let r = color.r();
    let g = color.g();
    let b = color.b();

    'line: for line in text.lines() {
        let mut glyph_x = x;

        'char: for char in line.chars() {
            let (metrics, bitmap) = font.rasterize(char, font_size as f32);

            let glyph_y = y as f32 - (metrics.height as f32 - metrics.advance_height) - metrics.ymin as f32;

            // if draw {
            'y: for y in 0..metrics.height {
                'x: for x in 0..metrics.width {
                    //Text doesn't fit on the screen.
                    if (x + glyph_x) >= window.width {
                        continue;
                    }

                    //TODO: Metrics.bounds determines the bounding are of the glyph.
                    //Currently the whole bitmap bounding box is drawn.
                    let alpha = bitmap[x + y * metrics.width];
                    if alpha == 0 {
                        continue;
                    }

                    //Should the text really be offset by the font size?
                    //This allows the user to draw text at (0, 0).
                    let offset = font_size as f32 + glyph_y + y as f32;

                    //We can't render off of the screen, mkay?
                    if offset < 0.0 {
                        continue;
                    }

                    if max_x < x + glyph_x {
                        max_x = x + glyph_x;
                    }

                    if max_y < offset as usize {
                        max_y = offset as usize;
                    }

                    if skip_draw {
                        continue;
                    }

                    let i = x + glyph_x + window.width * offset as usize;

                    if i >= buffer.len() {
                        break 'x;
                    }

                    let bg = Color(buffer[i]);

                    let r = blend(r, alpha, bg.r(), 255 - alpha);
                    let g = blend(g, alpha, bg.g(), 255 - alpha);
                    let b = blend(b, alpha, bg.b(), 255 - alpha);

                    if let Some(px) = buffer.get_mut(i) {
                        *px = rgb(r, g, b).as_u32();
                    }
                }
            }

            glyph_x += metrics.advance_width as usize;

            //Check if the glyph position is off the screen.
            if glyph_x >= window.width {
                break 'line;
            }
        }

        //CSS is probably line height * font size.
        //1.2 is the default line height
        //I'm guessing 1.0 is probably just adding the font size.
        y += font_size + line_height;
    }

    //Not sure why these are one off.
    area.height = max_y + 1 - area.y;
    area.width = max_x + 1 - area.x;

    area
}
