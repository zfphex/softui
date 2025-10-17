use crate::*;
use std::{ffi::OsStr, path::Path};
use taffy::{prelude::length, AlignItems, BoxSizing};
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

#[macro_export]
macro_rules! include_image {
    ($image:expr) => {{
        // let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $image));
        let bytes = include_bytes!($image);
        let ext = std::path::Path::new($image);
        image_bytes(ext, bytes)
    }};
}

pub fn image<'a>(image: &'a Image) -> ImageRef<'a> {
    ImageRef {
        format: image.format,
        layout: TaffyLayout {
            size: taffy::Size {
                width: length(image.area.width as f32),
                height: length(image.area.height as f32),
            },
            ..Default::default()
        },
        bitmap: &image.bitmap,
    }
}

pub fn image_bytes(ext: &Path, file: &[u8]) -> Image {
    let options = DecoderOptions::default()
        .png_set_strip_to_8bit(true)
        .png_set_add_alpha_channel(true);

    let ext = ext.extension().unwrap().to_string_lossy();
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

#[derive(Debug, Clone)]
pub struct Image {
    pub format: ImageFormat,
    pub area: Rect,
    pub bitmap: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ImageRef<'a> {
    pub format: ImageFormat,
    pub layout: TaffyLayout,
    pub bitmap: &'a [u8],
}

impl<'a> Widget<'a> for ImageRef<'a> {
    fn draw(&self, commands: &mut Vec<Command>, area: Rect, style: Option<Style>) {
        //TODO: Just assume the image exists for now.
        let bitmap = unsafe { std::mem::transmute::<&'a [u8], &'static [u8]>(self.bitmap) };

        commands.push(Command {
            area,
            primative: Primative::ImageUnsafe(bitmap, self.format),
        });
    }

    fn layout(&self) -> TaffyLayout {
        self.layout.clone()
    }
}
