use super::{Error, Result};
use cameleon::payload::{ImageInfo, PixelFormat};
use image::{Bgr, DynamicImage, GrayImage, ImageBuffer, RgbImage};

pub fn convert(buf: &[u8], info: &ImageInfo) -> Result<DynamicImage> {
    let buf = buf.to_vec();
    let width = info.width as u32;
    let height = info.height as u32;
    let pf = info.pixel_format;

    let image = match pf {
        PixelFormat::Mono8 => DynamicImage::ImageLuma8(
            GrayImage::from_raw(width, height, buf)
                .ok_or_else(|| Error::InvalidData("wrong image data".into()))?,
        ),
        PixelFormat::RGB8 => DynamicImage::ImageRgb8(
            RgbImage::from_raw(width, height, buf)
                .ok_or_else(|| Error::InvalidData("wrong image data".into()))?,
        ),
        PixelFormat::BGR8 => DynamicImage::ImageBgr8(
            ImageBuffer::<Bgr<u8>, _>::from_raw(width, height, buf)
                .ok_or_else(|| Error::InvalidData("wrong image data".into()))?,
        ),
        _ => {
            if buf.len() / (width * height) as usize == 1 {
                DynamicImage::ImageLuma8(
                    GrayImage::from_raw(width, height, buf)
                        .ok_or_else(|| Error::InvalidData("wrong image data".into()))?,
                )
            } else {
                return Err(Error::UnsupportedPixelFormat(pf));
            }
        }
    };
    Ok(image)
}
