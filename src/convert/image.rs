use super::{Error, Result};
use anyhow::anyhow;
use cameleon::payload::ImageInfo;
use cameleon_device::PixelFormat;
use image::{Bgr, DynamicImage, GrayImage, ImageBuffer, RgbImage};

macro_rules! failed {
    ($($arg: expr),*) => {
        Error::FailedConversion(anyhow!($($arg),*))
    };
}

#[cfg(not(feature = "cv"))]
pub fn convert_impl(buf: &[u8], info: &ImageInfo) -> Result<DynamicImage> {
    let buf = buf.to_vec();
    let width = info.width as u32;
    let height = info.height as u32;
    let pf = info.pixel_format;

    let image = match pf {
        PixelFormat::Mono8 => DynamicImage::ImageLuma8(
            GrayImage::from_raw(width, height, buf).ok_or_else(|| failed!("wrong image data"))?,
        ),
        PixelFormat::RGB8 => DynamicImage::ImageRgb8(
            RgbImage::from_raw(width, height, buf).ok_or_else(|| failed!("wrong image data"))?,
        ),
        PixelFormat::BGR8 => DynamicImage::ImageBgr8(
            ImageBuffer::<Bgr<u8>, _>::from_raw(width, height, buf)
                .ok_or_else(|| failed!("wrong image data"))?,
        ),
        _ => {
            if buf.len() / (width * height) as usize == 1 {
                DynamicImage::ImageLuma8(
                    GrayImage::from_raw(width, height, buf)
                        .ok_or_else(|| failed!("wrong image data"))?,
                )
            } else {
                return Err(failed!("unsupported pixel format: {:?}", pf));
            }
        }
    };
    Ok(image)
}
