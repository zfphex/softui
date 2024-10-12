use std::sync::Arc;

use dwrote::*;

use super::FONT;

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
    pub fn glyph(&self, char: char, point_size: f32) -> (i32, i32, Vec<u8>) {
        let glyph_id = self
            .font_face
            .get_glyph_indices(&[char as u32])
            .into_iter()
            .next()
            .and_then(|g| if g != 0 { Some(g as u32) } else { None })
            .unwrap();

        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: unsafe { self.font_face.as_ptr() },
            fontEmSize: point_size,
            glyphCount: 1,
            glyphIndices: &(glyph_id as u16),
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
            return (0, 0, Vec::new());
        }

        let alpha_texture = glyph_analysis
            .create_alpha_texture(DWRITE_TEXTURE_CLEARTYPE_3x1, texture_bounds)
            .unwrap();

        (texture_width, texture_height, alpha_texture)
    }
}
