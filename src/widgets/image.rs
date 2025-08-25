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

pub fn image_ref<'a>(image: &'a Image) -> ImageRef<'a> {
    ImageRef {
        format: image.format,
        size: image.area.into(),
        bitmap: &image.bitmap,
    }
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

#[derive(Debug, Clone)]
pub struct Image {
    pub format: ImageFormat,
    pub area: Rect,
    pub bitmap: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ImageRef<'a> {
    pub format: ImageFormat,
    pub size: Size,
    pub bitmap: &'a [u8],
}

impl<'a> Widget<'a> for ImageRef<'a> {
    fn position(&mut self, parent: Rect) {
        self.size = parent.into();
    }

    fn draw(&self, commands: &mut Vec<Command>, style: Option<Style>) {
        //TODO: Just assume the image exists for now.
        let bitmap = unsafe { std::mem::transmute::<&'a [u8], &'static [u8]>(self.bitmap) };

        commands.push(Command {
            area: self.size.clone().into_rect(),
            primative: Primative::ImageUnsafe(bitmap, self.format),
        });
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
    
    fn size(&mut self, _: Rect) {
    }
}
