//I don't like any of this code
//We should probably cache n most recent glyphs accessed.
//Do we ask dwrite to do layout too? I don't wanna.
use super::FONT;
use dwrote::*;
use mini::profile;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct Texture {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GlyphMetrics {
    pub advance_width: f32,
    pub advance_height: f32,
    pub bottom_side_bearing: f32,
    pub vertical_origin_y: f32,
    pub top_side_beaing: f32,
    pub right_side_bearing: f32,
    pub left_side_bearing: f32,
}

#[derive(Debug, Default)]
pub struct Glyph {
    pub texture: Texture,
    pub metrics: GlyphMetrics,
}

impl Glyph {
    pub const fn new() -> Self {
        Self {
            texture: Texture {
                data: Vec::new(),
                width: 0,
                height: 0,
            },
            metrics: const {
                GlyphMetrics {
                    advance_width: 0.0,
                    advance_height: 0.0,
                    bottom_side_bearing: 0.0,
                    vertical_origin_y: 0.0,
                    top_side_beaing: 0.0,
                    right_side_bearing: 0.0,
                    left_side_bearing: 0.0,
                }
            },
        }
    }
}

pub struct DWrite {
    pub font_face: FontFace,
    //How would we support multiple font sizes.
    //This is basically useless as it is.
    pub table: [Glyph; 127],
    pub ascent: f32,
    pub decent: f32,
    pub design_units: f32,
}

impl DWrite {
    pub fn new_cached(point_size: f32) -> Self {
        let mut dwrite = Self::new();

        let metrics = dwrite.font_face.metrics().metrics0();
        let em = metrics.designUnitsPerEm as f32;
        dwrite.design_units = em;
        let ratio = point_size / em;
        dwrite.ascent = metrics.ascent as f32 * ratio;
        dwrite.decent = metrics.descent as f32 * ratio;

        for char in 32..127 {
            let (metrics, texture) = dwrite.glyph(char as u8 as char, point_size);
            dwrite.table[char as usize] = Glyph { metrics, texture };
        }

        dwrite
    }
    //TODO: Too many nested structs bro.
    pub fn glyph_cached(&self, char: char) -> (GlyphMetrics, &Texture) {
        profile!();
        let glyph = &self.table[char as usize];
        (glyph.metrics, &glyph.texture)
    }
    pub fn new() -> Self {
        let font_data = Arc::new(FONT);
        let font_file = FontFile::new_from_buffer(font_data.clone()).unwrap();
        let collection_loader = CustomFontCollectionLoaderImpl::new(&[font_file.clone()]);
        let collection = FontCollection::from_loader(collection_loader);
        let families: Vec<_> = collection.families_iter().collect();
        let font = families[0].font(3).unwrap();
        let font_face = font.create_font_face();
        Self {
            font_face,
            table: [const { Glyph::new() }; 127],
            ascent: todo!(),
            decent: todo!(),
            design_units: todo!(),
        }
    }
    //TODO: I got bored half way through writing this.
    pub fn new_cached_2(font_size: f32) -> Self {
        let mut dwrite = Self::new();
        let glyphs: Vec<u32> = (32..127).collect();
        let glyph_ids: Vec<u16> = dwrite.font_face.get_glyph_indices(&glyphs);

        let metrics = dwrite.font_face.metrics().metrics0();
        let design_units = metrics.designUnitsPerEm as f32;

        for (index, gm) in dwrite
            .font_face
            .get_design_glyph_metrics(&glyph_ids, false)
            .iter()
            .enumerate()
        {
            // let glyph_metrics = GlyphMetrics {
            //     advance_width: gm.advanceWidth as f32 * ratio,
            //     advance_height: gm.advanceHeight as f32 * ratio,
            //     bottom_side_bearing: gm.bottomSideBearing as f32 * ratio,
            //     ascent: metrics.ascent as f32 * ratio,
            //     decent: metrics.descent as f32 * ratio,
            //     vertical_origin_y: ,
            // };
            // if char == ' ' {
            //     return (
            //         glyph_metrics,
            //         Texture {
            //             data: Vec::new(),
            //             width: glyph_metrics.advance_width as i32,
            //             height: 0,
            //         },
            //     );
            // }
            // let glyph_run = DWRITE_GLYPH_RUN {
            //     fontFace: unsafe { dwrite.font_face.as_ptr() },
            //     fontEmSize: point_size,
            //     glyphCount: 1,
            //     glyphIndices: &glyph_ids[index],
            //     glyphAdvances: &0.0,
            //     glyphOffsets: &GlyphOffset {
            //         advanceOffset: 0.0,
            //         ascenderOffset: 0.0,
            //     },
            //     isSideways: 0,
            //     bidiLevel: 0,
            // };

            // let glyph_analysis = GlyphRunAnalysis::create(
            //     &glyph_run,
            //     1.0,
            //     None,
            //     DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
            //     DWRITE_MEASURING_MODE_NATURAL,
            //     0.0,
            //     0.0,
            // )
            // .unwrap();

            // let texture_bounds = glyph_analysis
            //     .get_alpha_texture_bounds(DWRITE_TEXTURE_CLEARTYPE_3x1)
            //     .unwrap();
            // let texture_width = texture_bounds.right - texture_bounds.left;
            // let texture_height = texture_bounds.bottom - texture_bounds.top;

            // if texture_width == 0 || texture_height == 0 {

            //     return (
            //         glyph_metrics,
            //         Texture {
            //             data: Vec::new(),
            //             width: texture_width,
            //             height: texture_height,
            //         },
            //     );
            // }

            // let alpha_texture = glyph_analysis
            //     .create_alpha_texture(DWRITE_TEXTURE_CLEARTYPE_3x1, texture_bounds)
            //     .unwrap();

            // (
            //     glyph_metrics,
            //     Texture {
            //         data: alpha_texture,
            //         width: texture_width,
            //         height: texture_height,
            //     },
            // )
        }

        dwrite
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
        let design_units = metrics.designUnitsPerEm as f32;
        let ratio = point_size / design_units;

        let gm = self.font_face.get_design_glyph_metrics(&[glyph_id], false)[0];
        // dbg!(gm.advanceHeight, gm.bottomSideBearing,         gm.topSideBearing, gm.verticalOriginY);

        let glyph_metrics = GlyphMetrics {
            left_side_bearing: gm.leftSideBearing as f32 * ratio,
            advance_width: gm.advanceWidth as f32 * ratio,
            right_side_bearing: gm.rightSideBearing as f32 * ratio,
            top_side_beaing: gm.topSideBearing as f32 * ratio,
            advance_height: gm.advanceHeight as f32 * ratio,
            bottom_side_bearing: gm.bottomSideBearing as f32 * ratio,
            vertical_origin_y: gm.verticalOriginY as f32 * ratio,
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
