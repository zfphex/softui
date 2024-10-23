use crate::*;
use std::{ffi::OsStr, path::Path};
//TODO: Probably don't need the entire zune_image crate.
use zune_image::codecs::{
    jpeg::JpegDecoder,
    png::{zune_core::options::DecoderOptions, PngDecoder},
    ImageFormat,
};
use zune_image::traits::DecoderTrait;

pub fn image(path: impl AsRef<Path>) -> Image {
    let path = path.as_ref();
    let file = std::fs::read(path).unwrap();
    let options = DecoderOptions::default()
        .png_set_strip_to_8bit(true)
        .png_set_add_alpha_channel(true);

    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy();
            match &*ext {
                "jpg" | "jpeg" => {
                    let mut decoder = JpegDecoder::new_with_options(file, options);
                    let decode = decoder.decode().unwrap();
                    let (width, height) = decoder.dimensions().unwrap();

                    Image {
                        format: ImageFormat::JPEG,
                        area: Rect::new(0, 0, width as i32, height as i32),
                        bitmap: decode,
                    }
                }
                "png" => {
                    let mut decoder = PngDecoder::new_with_options(file, options);
                    let decode = decoder.decode().unwrap();
                    let (width, height) = decoder.dimensions().unwrap();

                    Image {
                        format: ImageFormat::PNG,
                        area: Rect::new(0, 0, width as i32, height as i32),
                        bitmap: decode.u8().unwrap(),
                    }
                }
                _ => panic!("{} is not a supported image extension.", ext),
            }
        }
        None => panic!("File has no extension and cannot be a valid image."),
    }
}

pub fn draw_image(image: &Image, mut x: usize, mut y: usize) {
    let ctx = ctx();

    let width = ctx.window.area().width;
    let buffer = &mut ctx.window.buffer();
    let len = buffer.len();

    let chunk_size = if image.format == ImageFormat::PNG {
        //4 bytes per channel rgba
        4
    } else {
        //3 bytes per channel rgb
        3
    };

    for pixel in image.bitmap.chunks(chunk_size) {
        let pos = y * width as usize + x;

        if pos >= len {
            break;
        }

        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        // let a = pixel[3];
        let color = rgb(r, g, b);

        buffer[pos] = color;

        x += 1;
        if x >= image.area.width as usize {
            y += 1;
            x = 0;
            continue;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub format: ImageFormat,
    pub area: Rect,
    pub bitmap: Vec<u8>,
}

impl Image {}

impl Widget for Image {
    fn area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn layout_area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }
}
