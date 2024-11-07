//I don't like any of this code
//We should probably cache n most recent glyphs accessed.
//Do we ask dwrite to do layout too? I don't wanna.
use super::FONT;
use dwrote::*;
use mini::profile;
use std::sync::Arc;

pub struct Texture {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct GlyphMetrics {
    // pub left_size_bearing: i32,
    pub advance_width: f32,
    // pub right_side_bearing: i32,
    // pub top_side_bearing: i32,
    pub advance_height: f32,
    pub bottom_side_bearing: f32,
    pub ascent: f32,
    pub decent: f32,
    // pub vertical_origin_y: i32,
}

pub struct Glyph {
    pub texture: Texture,
    pub metrics: GlyphMetrics,
}

pub struct DWrite {
    pub font_face: FontFace,
    //How would we support multiple font sizes.
    //This is basically useless as it is.
    pub table: [(Vec<Glyph>); 127],
}

impl DWrite {
    pub fn new_cached(point_size: f32) -> Self {
        let mut dwrite = Self::new();

        for char in 32..127 {
            //TODO: Cleanup
            let (metrics, texture) = dwrite.glyph(char as u8 as char, point_size);
            //Box cannot be const so this will do for now. It's just a pointer really.
            dwrite.table[char as usize] = vec![Glyph { texture, metrics }];
        }

        dwrite
    }
    //TODO: Too many nested structs bro.
    pub fn glyph_cached(&self, char: char) -> (GlyphMetrics, &Texture) {
        profile!();
        let glyph = &self.table[char as usize][0];
        (glyph.metrics, &glyph.texture)
    }
    pub fn new() -> Self {
        let font_data = Arc::new(FONT);
        let font_file = FontFile::new_from_buffer(font_data.clone()).unwrap();
        let collection_loader = CustomFontCollectionLoaderImpl::new(&[font_file.clone()]);
        let collection = FontCollection::from_loader(collection_loader);
        let family = collection.get_font_family(0);
        let font = family.get_font(3);
        let font_face = font.create_font_face();
        Self {
            font_face,
            table: [const { Vec::new() }; 127],
        }
    }
    pub fn glyph(&self, char: char, point_size: f32) -> (GlyphMetrics, Texture) {
        profile!();
        let glyph_id = self
            .font_face
            .get_glyph_indices(&[char as u32])
            .into_iter()
            .next()
            .and_then(|g| if g != 0 { Some(g) } else { None })
            .unwrap();

        let metrics = self.font_face.metrics().metrics0();
        let em = metrics.designUnitsPerEm as f32;
        let ratio = point_size / em;

        // let gm = self.font_face.get_gdi_compatible_glyph_metrics(
        //     point_size,
        //     1.0,
        //     core::ptr::null(),
        //     true,
        //     &[glyph_id],
        //     false,
        // )[0];

        let gm = self.font_face.get_design_glyph_metrics(&[glyph_id], false)[0];
        let glyph_metrics = GlyphMetrics {
            advance_width: gm.advanceWidth as f32 * ratio,
            advance_height: gm.advanceHeight as f32 * ratio,
            bottom_side_bearing: gm.bottomSideBearing as f32 * ratio,
            ascent: metrics.ascent as f32 * ratio,
            decent: metrics.descent as f32 * ratio,
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
            DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
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
