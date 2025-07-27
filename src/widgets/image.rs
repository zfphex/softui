use crate::*;
use std::{ffi::OsStr, path::Path};
//TODO: Probably don't need the entire zune_image crate.
use zune_image::codecs::{
    jpeg::JpegDecoder,
    png::{zune_core::options::DecoderOptions, PngDecoder},
};
use zune_image::traits::DecoderTrait;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ImageFormat {
    PNG,
    JPEG,
}

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
                    let bitmap = decoder.decode().unwrap();
                    let (width, height) = decoder.dimensions().unwrap();

                    Image {
                        format: ImageFormat::JPEG,
                        area: Rect::new(0, 0, width, height),
                        bitmap,
                    }
                }
                "png" => {
                    let mut decoder = PngDecoder::new_with_options(file, options);
                    let bitmap = decoder.decode().unwrap();
                    let (width, height) = decoder.dimensions().unwrap();

                    Image {
                        format: ImageFormat::PNG,
                        area: Rect::new(0, 0, width, height),
                        bitmap: bitmap.u8().unwrap(),
                    }
                }
                _ => panic!("{} is not a supported image extension.", ext),
            }
        }
        None => panic!("File has no extension and cannot be a valid image."),
    }
}

// pub fn draw_image(image: &Image, mut x: usize, mut y: usize) {
//     let ctx = ctx();

//     let width = ctx.window.area().width;
//     let buffer = &mut ctx.window.buffer();
//     let len = buffer.len();

//     let chunk_size = if image.format == ImageFormat::PNG {
//         //4 bytes per channel rgba
//         4
//     } else {
//         //3 bytes per channel rgb
//         3
//     };

//     for pixel in image.bitmap.chunks(chunk_size) {
//         let pos = y * width as usize + x;

//         if pos >= len {
//             break;
//         }

//         let r = pixel[0];
//         let g = pixel[1];
//         let b = pixel[2];
//         // let a = pixel[3];
//         let color = rgb(r, g, b);

//         buffer[pos] = color;

//         x += 1;
//         if x >= image.width as usize {
//             y += 1;
//             x = 0;
//             continue;
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Image {
    pub format: ImageFormat,
    pub area: Rect,
    pub bitmap: Vec<u8>,
}

impl Image {}

impl<'a> Widget<'a> for Image {
    fn size(&self) -> (usize, usize) {
        (self.area.width, self.area.height)
    }
    fn layout(&mut self, area: Rect) {
        self.area = area;
    }
    fn area_mut(&mut self) -> &mut Rect {
        &mut self.area
    }
    fn handle_event(&mut self, _ctx: &mut Context) {}
    fn draw(&self, commands: &mut Vec<Command>) {
        //TODO: Just assume the image exists for now.
        let bitmap = unsafe { extend_lifetime(&self.bitmap) };
        commands.push(Command {
            area: self.area,
            primative: Primative::ImageUnsafe(bitmap, self.format),
        });
    }
}
