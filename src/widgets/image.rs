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
                        area: Rect::new(0, 0, width as i32, height as i32),
                        bitmap,
                    }
                }
                "png" => {
                    let mut decoder = PngDecoder::new_with_options(file, options);
                    let bitmap = decoder.decode().unwrap();
                    let (width, height) = decoder.dimensions().unwrap();

                    Image {
                        format: ImageFormat::PNG,
                        area: Rect::new(0, 0, width as i32, height as i32),
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

impl Widget for Image {
    fn width<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        unimplemented!("The width of an image is fixed");
    }

    fn height<U: Into<Unit>>(mut self, length: U) -> Self
    where
        Self: Sized,
    {
        unimplemented!("The height of an image is fixed");
    }

    fn area(&mut self) -> Option<&mut Rect> {
        Some(&mut self.area)
    }

    fn draw_command(&self) -> Option<Command> {
        //TODO: Just assume the image exists for now.
        let bitmap = unsafe { extend_lifetime(&self.bitmap) };
        Some(Command::ImageUnsafe(
            bitmap,
            self.area.x as usize,
            self.area.y as usize,
            self.area.width as usize,
            self.area.height as usize,
            self.format,
        ))
        // Some(Command::Image(
        //     self.bitmap.clone().into_boxed_slice(),
        //     self.area.x as usize,
        //     self.area.y as usize,
        //     self.width,
        //     self.height,
        //     self.format,
        // ))
    }
}

unsafe fn extend_lifetime<'a>(i: &'a [u8]) -> &'static [u8] {
    std::mem::transmute::<&'a [u8], &'static [u8]>(i)
}
