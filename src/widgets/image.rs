use crate::*;
use codecs::png::{zune_core::options::DecoderOptions, PngDecoder};
use std::path::Path;
use traits::DecoderTrait;
use zune_image::*;

pub fn image(path: impl AsRef<Path>) -> Image {
    // let image = zune_image::image::Image::open(path).unwrap();
    // let (width, height) = image.dimensions();
    // let vec = image.write_to_vec(codecs::ImageFormat::BMP).unwrap();

    let file = std::fs::read(path).unwrap();
    let options = DecoderOptions::default()
        .png_set_strip_to_8bit(true)
        .png_set_add_alpha_channel(true);
    let mut decoder = PngDecoder::new_with_options(file, options);
    let decode = decoder.decode().unwrap();
    let (width, height) = decoder.dimensions().unwrap();

    Image {
        width,
        height,
        bitmap: decode.u8().unwrap(),
    }
}

pub fn draw_png(image: &Image) {
    // let bitmap = unsafe { image.bitmap.align_to::<u32>().1 };
    let ctx = ctx();

    let mut x = 0;
    let mut y = 0;

    for pixel in image.bitmap.chunks(4) {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        // let a = pixel[3];
        ctx.draw_pixel(x, y, rgb(r, g, b));

        x += 1;
        if x >= image.width {
            y += 1;
            x = 0;
            continue;
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub bitmap: Vec<u8>,
}

impl Image {}
impl Widget for Image {
    fn area(&mut self) -> Option<&mut Rect> {
        todo!()
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        todo!()
    }
}
