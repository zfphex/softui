use std::sync::Arc;

use dwrote::*;

use super::FONT;

pub struct Texture {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone)]
pub struct Metrics {
    pub left_size_bearing: i32,
    pub advance_width: u32,
    pub right_side_bearing: i32,
    pub top_side_bearing: i32,
    pub advance_height: u32,
    pub bottom_side_bearing: i32,
    pub vertical_origin_y: i32,
}

pub struct DWrite {
    font_face: FontFace,
}

impl DWrite {
    pub fn new() -> Self {
        let font_data = Arc::new(FONT);
        let font_file = FontFile::new_from_buffer(font_data.clone()).unwrap();
        let collection_loader = CustomFontCollectionLoaderImpl::new(&[font_file.clone()]);
        let collection = FontCollection::from_loader(collection_loader);
        let family = collection.get_font_family(0);
        let font = family.get_font(3);
        let font_face = font.create_font_face();
        Self { font_face }
    }
    pub fn glyph(&self, char: char, point_size: f32) -> (Metrics, Texture) {
        let glyph_id = self
            .font_face
            .get_glyph_indices(&[char as u32])
            .into_iter()
            .next()
            .and_then(|g| if g != 0 { Some(g) } else { None })
            .unwrap();

        let glyph_metrics = self.font_face.get_design_glyph_metrics(&[glyph_id], false)[0];
        let metrics = self.font_face.metrics().metrics0();

        //object oriented programming at it's finest 🍷
        let glyph_metrics = Metrics {
            left_size_bearing: ((glyph_metrics.leftSideBearing as f32
                / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as i32,
            advance_width: ((glyph_metrics.advanceWidth as f32 / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as u32,
            right_side_bearing: ((glyph_metrics.rightSideBearing as f32
                / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as i32,
            top_side_bearing: ((glyph_metrics.topSideBearing as f32
                / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as i32,
            advance_height: ((glyph_metrics.advanceHeight as f32 / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as u32,
            bottom_side_bearing: ((glyph_metrics.bottomSideBearing as f32
                / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as i32,
            vertical_origin_y: ((glyph_metrics.verticalOriginY as f32
                / metrics.designUnitsPerEm as f32)
                * point_size)
                .round() as i32,
        };

        if char == ' ' {
            return (
                glyph_metrics,
                Texture {
                    data: Vec::new(),
                    width: glyph_metrics.advance_width as i32,
                    height: 0,
                },
            );
        }

        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: unsafe { self.font_face.as_ptr() },
            fontEmSize: point_size,
            glyphCount: 1,
            glyphIndices: &glyph_id,
            glyphAdvances: &0.0,
            glyphOffsets: &GlyphOffset {
                advanceOffset: 0.0,
                ascenderOffset: 0.0,
            },
            isSideways: 0,
            bidiLevel: 0,
        };
        let glyph_analysis = GlyphRunAnalysis::create(
            &glyph_run,
            1.0,
            None,
            DWRITE_RENDERING_MODE_NATURAL,
            DWRITE_MEASURING_MODE_NATURAL,
            0.0,
            0.0,
        )
        .unwrap();

        let texture_bounds = glyph_analysis
            .get_alpha_texture_bounds(DWRITE_TEXTURE_CLEARTYPE_3x1)
            .unwrap();
        let texture_width = texture_bounds.right - texture_bounds.left;
        let texture_height = texture_bounds.bottom - texture_bounds.top;

        if texture_width == 0 || texture_height == 0 {
            return (
                glyph_metrics,
                Texture {
                    data: Vec::new(),
                    width: texture_width,
                    height: texture_height,
                },
            );
        }

        let alpha_texture = glyph_analysis
            .create_alpha_texture(DWRITE_TEXTURE_CLEARTYPE_3x1, texture_bounds)
            .unwrap();

        (
            glyph_metrics,
            Texture {
                data: alpha_texture,
                width: texture_width,
                height: texture_height,
            },
        )
    }
}
